# Structured Concurrency in Raro

## Examples

### Server with background and connection tasks

```rust
'pipeline' import as pipeline;
{
  listen{ http-handle{; 'hello' } await? } await?
} as api-server;
[
  pipeline
  api-server
] all await?;
```

The application has two top level tasks: `pipeline` and `api-server`. The server has a listener which spawns tasks into 

### Many HTTP requests

```rust
[
  "https://a.example/",
  "https://b.example/",
  "https://c.example/",
] as urls;

# Wait for every response
urls map { http.get(_) } all as bodies;

# Or race for the first result
urls map { http.get(_) } race as first-body;
```

In both forms, if any request errors, all in-flight requests are cancelled.
`race` and `all` differ only in how branches are weighted.

## Takeaway

Because scopes are never forgotten and cancellation is enforced, Raro can safely
support **parallelism + borrowing + structured joins**, something that Rust's
model cannot provide without compromise. Structured concurrency is baked into
the language rather than left to ad-hoc libraries.
