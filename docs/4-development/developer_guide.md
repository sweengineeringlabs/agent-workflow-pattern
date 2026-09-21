# Agent Workflow Developer Guide

## Overview

Day-to-day development guide for working on `agent-workflow-pattern`.

## Module Structure

```
agent-workflow-pattern/
└── scm/
    ├── Cargo.toml              # zero [dependencies]
    └── src/
        ├── lib.rs               # #![deny(unsafe_code)] #![warn(missing_docs)]
        ├── traits/
        │   ├── mod.rs            # mod pattern; pub use pattern::*;
        │   └── pattern.rs        # Workflow, WorkflowStep, WorkflowStepExecutor, WorkflowStepRouting
        └── vo/
            └── mod.rs             # empty -- everything fits in traits/pattern.rs
```

Note the crate's Rust package root is `scm/` (Cargo.toml/src live there), not the repo root —
every command below assumes `cd scm` first.

## Development Workflow

1. **Branch** from `main` (this repo has no `dev` branch — every commit lands directly on `main`).
2. **Implement** changes in `src/traits/pattern.rs` (or split into a new file under `traits/` if
   a genuinely new trait is added — see [Common Tasks](#common-tasks)).
3. **Test**: this crate ships no implementation, so it has no unit tests of its own by design —
   real coverage lives in `agent-workflow-svc`'s integration tests, which construct a real
   `Workflow` and exercise this crate's trait contract end to end.
4. **Lint**: `cd scm && cargo build --all-targets && cargo clippy --all-targets -- -D warnings && cargo fmt --all -- --check`
5. **Dependency audit**: `cd scm && cargo deny check` (advisories + licenses + bans + sources)
6. CI (`.github/workflows/ci.yml`) runs all of the above automatically on every push/PR to
   `main` — added this session, previously this repo (like every `agent-*` repo except
   `agent-spec-api`) had no CI at all.
7. **Submit** a pull request targeting `main`.

## Key Conventions

### Zero implementation, zero dependencies

This is a **contract crate** — see
[`pattern_svc_workflow.md`](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)'s
"zero implementation" rule. Before adding any code here, ask: does this implement `Workflow` or
`WorkflowStep` for a concrete type? If yes, it belongs in `agent-workflow-svc`, not here. Small
structural `impl`s on this crate's own value types (a hand-written `Display`, a constructor) are
fine; implementing a *primary trait* is not.

### Naming

| Item | Convention | Example |
|------|-----------|---------|
| Traits | PascalCase noun | `Workflow`, `WorkflowStep` |
| Enum variants | PascalCase | `WorkflowStepExecutor::Prompt`, `WorkflowStepRouting::Done` |
| Functions | snake_case, verb-first | `step_ids`, `supports_variable_substitution` |

### Error Handling

No `error/` module — this contract declares no fallible operation of its own. `Workflow`/
`WorkflowStep`'s methods are all infallible accessors; a real implementor's *construction* may
fail (`agent-workflow-svc`'s `WorkflowBuilder::build() -> Result<impl Workflow, String>`), but
that failure mode belongs to the `-svc` side, not this contract.

## Common Tasks

### Adding a new accessor to `Workflow`/`WorkflowStep`

1. Add the method to the trait in `src/traits/pattern.rs`. If it can have a sensible default
   (matching every other domain's own precedent, e.g. `supports_variable_substitution`'s `true`
   default), give it one — that keeps every existing real implementor compiling without a
   breaking change.
2. Bump `Cargo.toml`'s version: additive (a new method with a default) is a patch bump; a new
   *required* method (no default) is a breaking, minor-version bump — see
   `pattern_svc_workflow.md`'s SemVer table.
3. Implement it in `agent-workflow-svc`'s `DefaultWorkflow`/`WorkflowStepImpl`.

### Adding a genuinely new domain concept

If the new concept has a different consumer or a different reason to change than
`Workflow`/`WorkflowStep` (see `pattern_svc_workflow.md`'s Single Responsibility test), it is a
new `-pattern`/`-svc` repo pair, not a third trait bundled in here.

## See Also

- [Architecture](../3-design/architecture.md)
- [pattern_svc_workflow.md](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)
- [agent-workflow-svc](https://github.com/sweengineeringlabs/agent-workflow-svc)
