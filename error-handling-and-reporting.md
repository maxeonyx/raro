# Error handling versus error reporting

Programming languages often muddle two different responsibilities under the
umbrella of "error handling": reacting to a recoverable failure in the control
flow, and reporting a diagnostic to an operator. As Matklad points out in
[Error Codes for Control Flow](https://matklad.github.io/2025/11/06/error-codes-for-control-flow.html),
these concerns usually travel to different destinations, so coupling them is
needless friction.

Raro’s effect system is well-suited for expressing the control-flow side —
deciding whether to retry, substitute a default value, or propagate the failure.
Separately, we should design diagnostic types that can be rendered in multiple
contexts (terminal, logs, web UIs) without dictating control flow. Some failures
will be handled and never reported; others will be reported because there is no
recovery path. Raro should make both straightforward without forcing them into
the same abstraction.

This separation also invites a richer diagnostic story: effects give us hooks to
capture contextual information for reporting (backtraces, metadata, localized
messages) without constraining how control flow resumes. Thinking about the two
concerns independently lets us design each to excel at its own job while still
working together when needed.
