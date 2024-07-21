# RARO

Return to the intuitive thought patterns of your pre-sullied mind.

## `R`unning `Ar`ithmetic with `O`bjects

**Have you ever done maths like this?**

```python
1 + 2 = 3

+ 3 = 6

+ 9 = 15
```

Did your teachers at school brainwash you into a 6-letter acronym instead?

- B.O.D.M.A.S.
- P.E.M.D.A.S.
- B.I.D.M.A.S.
- B.A.D.A.S.S.
- D.A.D.B.O.D.
- B.E.D.M.A.S.

Enough of such gaslighting. Free yourself from order. Never
retrace your steps.

*RARO* is an "append only" programming language which lets you program in the same
way that you think - forward!

## Running Arithmetic

```python
>>> 1
1
```

```python
>>> 1 + 2
3
```

```python
>>> 1 + 2 * 3
9
```

✅✅✅

## With Objects

```python
>>> { a: 1 }.a
1
```

```python
>>> { a: 1 }.b try + 2
Error( ... )
>>> { a: 1 }.a try + 2
3
```

## Quirks

### Append variable bindings anywhere with `as`

```python
>>> 1 + 2 as three + 4 as seven
7
>>> three + seven
10
```

### Functions have one implicit argument

(Similar to [Nix](https://nix.dev/manual/nix/2.18/language/), but even more terse)

```rust
>>> fn foo {
...     ## Adds two to its argument.
...     + 2
... }
<function>
```

That is just syntactic sugar for:

```rust
>>> { + 2 } as foo
<function>
```

But, we can unpack the default argument to simulate multiple parameters:

```rust
>>> fn bar {
...     as { a, b, c }; # Expects the implicit argument to be a map, and binds
...                     # keys a, b, c to variables.
...     ## Use "FEMDAD" or whatever with ( )
...     a + (b * c)
... }
<function>
```

### `[]` is an *un-ordered* set

```python
>>> [ 1, 2, 3, 4, 5 ]
[ 4, 2, 1, 3, 5 ]
# Order is randomized in debug mode, to prevent the programmer from accidentally relying on it (via side-effects).
```

```python
# map built-in takes an anonymous function
>>> [ 1, 2, 3, 4, 5 ] map { + 2 }
[ 7, 4, 5, 6, 3 ]
```

We recognize that ordering is a commonly observable side effect of parallelism,
and prevent the programmer from observing it. This reserves the ability to safely
parallelize the `map` built-in.

If ordering is needed, use a map with numeric keys:

```rust
>>> { 1: 1, 2: 2, 3: 3, 4: 4, 5: 5 } as ordered
{ 1: 1, 2: 2, 3: 3, 4: 4, 5: 5 }
# map is shown sorted by its keys

# `map` over an ordered set (a `struct`) provides { k: str, v: str } pair structs.
>>> ordered map { .v + 2 } # . is simply an operator on the previous expression, implicitly a struct.
{ 1: 3, 2: 4, 3: 5, 4: 6, 5: 7 }
```

Ordering is needed much less than you think when programming in a functional
style.
