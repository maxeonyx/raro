# Structured Concurrency in Raro

Structured concurrency ensures that when a block of concurrent tasks finishes, all
of its children have either completed or been cancelled before control continues.
Raro ties the lifetime of tasks to lexical scopes, so work cannot "leak" or
outlive its parent stream.

## Why Rust Struggles

In Rust, combining **parallelism**, **borrowing**, and **structured joins** is a
trilemma:

1. **Structured join** – the scope guarantees all tasks finish before returning.
2. **Borrowing from parent** – tasks can safely reference local stack data.
3. **Parallelism** – tasks make progress independently on other threads.

Rust can provide any two but not all three. Futures can be dropped or leaked,
scopes cannot enforce joining at `Drop`, and cancellation is cooperative only.

## What Raro Provides

Raro's runtime semantics close these gaps:

* **Uncancellable scopes** – blocks are never forgotten or leaked. When a
  structured block finishes, all branches are joined.
* **Mandatory cancellation on error** – if one branch fails, all siblings are
  cancelled. This makes cancellation a first-class effect.
* **Cancellation as effect** – a `cancelled` exception may be raised at any
  suspension point or even inside CPU-bound code. Critical sections can be
  marked as shielded to suppress this effect.
* **Parallelism with borrowing** – because scopes cannot be dropped and
  cancellation is strong, tasks may borrow from their parent while running in
  parallel.

## `all` and `race`

Raro offers two structured operators:

* `all` runs its branches concurrently and resumes the parent only after all
  branches finish. Results are merged with equal weighting.
* `race` runs the same way but resolves with the first completed branch,
  using branch weights to pick a winner. Unchosen branches are cancelled.

Both operators share identical cancellation behaviour: if any branch errors, all
siblings are cancelled; cancellation propagates uniformly across async I/O and
synchronous code. In effect, `all` is just `race` with uniform weights.

## Examples

### Server with background and connection tasks

```rust
listen "0.0.0.0:8080" as sock

all {
  metrics_daemon()

  loop {
    sock.accept as conn
    spawn handle(conn)
  }
}
```

The `metrics_daemon` lives for the duration of the application.  Each
`handle` task is spawned inside the server loop and is cancelled when the
server ends. The top-level `all` waits for both the background task and the
server to finish.

### Many HTTP requests

```rust
urls as [
  "https://a.example/",
  "https://b.example/",
  "https://c.example/",
]

# Wait for every response
urls map { http.get(_) } all as bodies

# Or race for the first result
urls map { http.get(_) } race as first_body
```

In both forms, if any request errors, all in-flight requests are cancelled.
`race` and `all` differ only in how branches are weighted.

## Takeaway

Because scopes are never forgotten and cancellation is enforced, Raro can safely
support **parallelism + borrowing + structured joins**, something that Rust's
model cannot provide without compromise. Structured concurrency is baked into
the language rather than left to ad-hoc libraries.
