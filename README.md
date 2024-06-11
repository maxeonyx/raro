# RARO

Return to the intuitive thought patterns of your pre-sullied mind.

## `R`unning `Ar`ithmetic with `O`bjects

**Have you ever done maths like this?**

```
1 + 2 = 3

+ 3 = 6

+ 9 = 15
```

Did your teachers at school brainwash you into a 6-letter acronym instead?

- B.O.D.M.A.S.
- P.E.M.D.A.S.
- B.I.D.M.A.S.
- B.A.D.A.S.S.
- P.E.N.N.A.S.
- B.E.D.M.A.S.

Enough of such gaslighting. Free yourself from order of operations. Never
retrace your steps.

*RARO* is an "append only" programming language which lets you program in the same
way that you think - forward!

# Running Arithmetic

```
>>> 1
1
```

```
>>> 1 + 2
3
```

```
>>> 1 + 2 * 3
9
```

```
>>> 1 + 2 as three + 4 as seven
7
>>> three + seven
10
```

# With Objects

```
>>> { a: 1 }.a
1
```

```
>>> { a: 1 }.b try + 2
Error( ... )
>>> { a: 1 }.a try + 2
3
```

# Quirks

## Functions have one implicit argument

```
fn foo {
    ## Adds two to its argument.
    + 2
}
```

But, we can unpack it:

```
fn bar {
    as { a, b, c }; # Expects the implicit argument to be a map, and binds
                    # keys a, b, c to variables.
    ## Breaks "FEMDAD" or whatever
    a + b * c
}
```

## `[]` is an un-ordered set

```
>>> [ 1, 2, 3, 4, 5 ] map { + 2 }
[ 7, 4, 5, 6, 3 ]
# or any other order. Randomized in debug mode.
```

We recognize that ordering is a commonly observable side effect of parallelism,
and prevent the programmer from observing it. This reserves the ability to safely
paralellize the `map` built-in.

If ordering is needed, use a map with numeric keys:

```
>>> { 1: 1, 2: 2, 3: 3, 4: 4, 5: 5 } map { as { k, v }; { k: k, v: v + 2 } }
{ 1: 3, 2: 4, 3: 5, 4: 6, 5: 7 }
```

Ordering is needed much less than you think when programming in a functional
style.
