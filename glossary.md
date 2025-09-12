# Raro Glossary

Translations between Raro idioms and more conventional languages like Python.

| Concept | Raro | Python | Notes |
| --- | --- | --- | --- |
| Function call | `x fib` | `fib(x)` | Raro uses postfix function application. |
| Infix arithmetic | `n - 1` | `n - 1` | Operators remain infix with no precedence; group with parentheses. |
| No precedence example | `n - 1 * 3` | `(n - 1) * 3` | Raro evaluates left-to-right, so Python needs parentheses to match. |
| Conditional | `cond if { ... } else { ... }` | `if cond: ... else: ...` | `if` follows the condition. |
| Loop over range | `{ start:1, end:100 } range each{ print }` | `for n in range(1, 101): print(n)` | No space before the brace passes the function as an argument. |
| Binding | `expr as name` | `name = expr` | `as` appends a binding to the stream. |
| Higher-order map | `[ 1, 2, 3 ] (print map) set` | `list(map(print, [1,2,3]))` | `map` yields a stream; `set` collects results. Parentheses make `print` the stream value. |
| Kebab-case identifier | `my-var` | `my_var` / `myVar` | `a-b` is one name; `a - b` subtracts. |

### Higher-order example

```
{ * 5 } as times-five;
{ as fn; { as value; [ value fn, value fn ] } } as apply-twice;
2 times-five apply-twice; # => [ 10, 10 ]
```

This showcases function values and postfix application.

