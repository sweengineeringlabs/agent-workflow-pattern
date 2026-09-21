# Agent Workflow

> **TLDR:** The Workflow pattern for agents — `Workflow`/`WorkflowStep` (a directed graph of
> steps with conditional routing), `WorkflowStepExecutor`/`WorkflowStepRouting` (how a step
> runs, where it goes next). Zero implementation, zero dependencies. See
> [Overview](docs/3-design/architecture.md) for details.

## Quick Start

```rust
use agent_workflow_pattern::{Workflow, WorkflowStep, WorkflowStepExecutor, WorkflowStepRouting};

fn describe(workflow: &dyn Workflow) -> String {
    format!(
        "{}: {} step(s), starts at '{}'",
        workflow.id(),
        workflow.step_ids().len(),
        workflow.start_step_id()
    )
}
```

`Workflow`/`WorkflowStep` are traits with no implementation here — a real, runnable
implementation lives in [`agent-workflow-svc`](https://github.com/sweengineeringlabs/agent-workflow-svc).

## API

| Type | Description |
|------|-------------|
| `Workflow` | An agent workflow — id, description, steps, routing, start step |
| `WorkflowStep` | A single step — prompt, executor, params, turn/iteration limits, routing |
| `WorkflowStepExecutor` | How a step runs: `Prompt` (LLM call), `HumanGate` (approval), `A2ADispatch` |
| `WorkflowStepRouting` | Where a step routes next: `Next(id)`, `Done`, `Failed` |

## Documentation

| Document | Description |
|----------|-------------|
| [Architecture](docs/3-design/architecture.md) | Component shape, zero-implementation rule |
| [Developer Guide](docs/4-development/developer_guide.md) | Module structure, workflow, conventions |
| [agent-workflow-svc](https://github.com/sweengineeringlabs/agent-workflow-svc) | Real implementation: `DefaultWorkflow`, `WorkflowFactory` |

## License

MIT OR Apache-2.0
