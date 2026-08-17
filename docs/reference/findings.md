# Findings

Every finding carries a code, a message, a path, and where applicable the feature and requirement it
belongs to. The prefix tells you which artifact owns the problem.

Fix the artifact that owns it. Never suppress a separation finding by moving requirement text to
another production file.

## `SPEC_` discovery and scope

| Code | Meaning |
| --- | --- |
| `SPEC_SCOPE` | The run evaluated one feature rather than the project. Pass `--all`. |
| `SPEC_NONE` | No specifications matched the configured glob. |
| `SPEC_MISSING` | A selected feature names a specification that does not exist. |
| `SPEC_UNREADABLE` | A specification could not be decoded and was not read. |
| `SPEC_OUTSIDE_PROJECT` | `--feature` resolved outside the project and was refused. |

## `EARS_` requirement form

| Code | Meaning |
| --- | --- |
| `EARS_PREFIX` | The sentence does not open with a recognised EARS form. |
| `EARS_SHALL` | Missing `shall`, or more than one obligation in a single requirement. |
| `EARS_MODAL` | A competing modal verb appears in a normative sentence. |
| `EARS_CLAUSE` | A When, While, or Where clause is not closed with a comma. |
| `EARS_UNWANTED` | An If form is missing its `then`. |
| `EARS_INCOMPLETE` | The requirement has no response after its trigger. |
| `REQ_DUPLICATE` | Two requirements in one feature share an identifier. |
| `REQ_NONE` | The specification declares no requirements. |

## `TRACE_` verification mapping

| Code | Meaning |
| --- | --- |
| `TRACE_MISSING` | No traceability file beside the specification. |
| `TRACE_MISSING_REQ` | A requirement has no entry. |
| `TRACE_UNKNOWN_REQ` | An entry names a requirement the specification does not declare. |
| `TRACE_MODE` | `verification` is neither `automated` nor `manual`. |
| `TRACE_MANUAL` | Manual verification without a concrete rationale. |
| `TRACE_TESTS` | Automated verification with no test selectors. |
| `TRACE_TEST_FILE` | A named test file does not exist. |
| `TRACE_TEST_NAME` | The file exists but does not contain the named test. |
| `TRACE_TEST_ROOT` | A selector points outside the configured test roots. |
| `TRACE_INVALID` | The file could not be parsed. |

## `TASK_` coverage

| Code | Meaning |
| --- | --- |
| `TASK_UNCOVERED` | A requirement is covered by no task. |
| `TASK_LIST_MISSING` | No `tasks.md` for the feature. |
| `TASK_UNKNOWN_REF` | A task references a requirement that does not exist. |
| `TASK_SPAN` | A task's requirement range could not be read. |

## `VOCAB_`, `TERM_`, `INTENT_` grounding

| Code | Meaning |
| --- | --- |
| `TERM_UNDECLARED` | A requirement tags a term the vocabulary does not declare. |
| `TERM_UNUSED` | A declared term is used by no requirement. |
| `TERM_DEPRECATED` | A requirement uses a term marked deprecated. Names the replacement. |
| `VOCAB_INVALID` | The vocabulary file could not be parsed, or a term redeclares a domain. |
| `VOCAB_CYCLE` | The `broader` relation contains a cycle. |
| `INTENT_UNDECLARED` | A requirement names an intention that does not exist. |
| `INTENT_UNSERVED` | A declared intention is served by no requirement. |
| `INTENT_INVALID` | The intentions file could not be parsed. |
| `INTENT_PRECEDENCE_CYCLE` | The precedence relation contains a cycle and resolves nothing. |
| `INTENT_PRECEDENCE_UNDECLARED` | Precedence names an intention that does not exist. |

## `MODEL_` within one specification

| Code | Meaning |
| --- | --- |
| `MODEL_CONFLICT` | Two requirements contradict each other. |
| `MODEL_CONFLICT_DEFECT` | They contradict and serve the same intention, so no precedence can help. |
| `MODEL_CONFLICT_ADJUDICATED` | They contradict and precedence names a winner. Advisory. |
| `MODEL_CONFLICT_UNADJUDICATED` | They contradict and nothing says which wins. |
| `MODEL_BUDGET_EXCEEDED` | A component is larger than the declared budget and was not searched. |
| `MODEL_DEAD_GUARD` | A guard can never be true, so the requirement never applies. |
| `MODEL_SUBSUMED` | A requirement's guard is implied by another's, making it redundant. |
| `MODEL_GUARD_INVALID` | The guard is outside the accepted language. |
| `MODEL_TYPE_MISMATCH` | A term is used in a way its declared domain does not support. |
| `MODEL_EFFECT_UNDECLARED` | A requirement asserts an effect the model does not declare. |
| `MODEL_UNKNOWN_REQ` | The model names a requirement the specification does not declare. |
| `MODEL_UNMODELLED` | A requirement has no model entry and was not checked. |
| `MODEL_INVALID` | The model file could not be parsed. |
| `MODEL_INTERNAL` | An internal contract failed. Please report it. |

## `MERGE_` across specifications

The same classifications, applied to the merged constraint system. These are the ones no review
catches.

| Code | Meaning |
| --- | --- |
| `MERGE_CONFLICT` | Two requirements in different specifications contradict each other. |
| `MERGE_CONFLICT_DEFECT` | They contradict and serve the same intention. |
| `MERGE_CONFLICT_ADJUDICATED` | Precedence names a winner. Advisory. |
| `MERGE_CONFLICT_UNADJUDICATED` | Nothing says which wins. |
| `MERGE_SHADOW` | One specification's requirement is subsumed by another's. |
| `MERGE_UNMERGED` | A specification declares no model and was excluded from the merge. |

Conflict findings carry a `witness` in their detail: a concrete assignment of terms that satisfies
both guards. See [finding contradictions](../concepts/contradictions.md).

## `CODE_` and `SEPARATION_` production code

| Code | Meaning |
| --- | --- |
| `CODE_REQ_ID` | A requirement identifier appears in production code. |
| `CODE_REQ_PROSE` | Requirement prose was copied into production code. |
| `SEPARATION_EXEMPT` | A finding was declared intentional with a reason. Advisory. |
| `SEPARATION_EXEMPT_NO_REASON` | An exemption marker carries no usable reason. |
| `SEPARATION_EXEMPT_STALE` | An exemption names a pattern that no longer matches. |
| `SEPARATION_EXEMPT_UNUSED` | An exemption matches nothing and should be removed. |
| `SEPARATION_INTERNAL` | The scan could not complete over a file. |

## `TEST_COMMAND`

The project has not declared its real `test_command` in `.specify/ears-sdd.toml`. The final gate
checks the command is declared. It never runs it.
