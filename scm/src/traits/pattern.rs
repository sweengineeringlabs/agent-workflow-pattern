//! Workflow pattern traits — orchestrate goal-oriented agent steps.

use std::collections::HashMap;

/// A single step in a workflow — what to do, how to execute it, and where to go next.
pub trait WorkflowStep: Send + Sync {
    /// Unique step identifier (e.g., "propose", "discuss", "approve").
    fn id(&self) -> &str;

    /// Prompt or instruction for this step.
    fn prompt(&self) -> &str;

    /// Executor type: how this step runs (prompt, human_gate, a2a_dispatch).
    fn executor(&self) -> WorkflowStepExecutor;

    /// Executor-specific parameters (e.g., timeout_secs for human_gate).
    fn params(&self) -> &HashMap<String, String>;

    /// Maximum turns this step can take (0 = no limit).
    fn max_turns(&self) -> u32;

    /// Maximum iterations if step can loop (0 = no limit).
    fn max_iterations(&self) -> u32;

    /// Routing after successful step completion.
    fn on_complete(&self) -> WorkflowStepRouting;

    /// Routing if step fails.
    fn on_fail(&self) -> WorkflowStepRouting;

    /// Routing on condition failure (e.g., human_gate rejection).
    fn on_condition_fail(&self) -> Option<WorkflowStepRouting>;
}

/// How a step is executed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowStepExecutor {
    /// Execute the prompt directly (LLM call).
    Prompt,
    /// Human approval gate (blocks until human responds, has timeout).
    HumanGate,
    /// Dispatch to another agent via A2A (Agent-to-Agent).
    A2ADispatch,
}

/// Where to route after a step completes or fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowStepRouting {
    /// Go to the next step by id.
    Next(String),
    /// Workflow completed successfully.
    Done,
    /// Workflow failed; abort and report error.
    Failed,
}

/// An agent workflow — orchestrates steps, routing, and state.
///
/// A workflow is a directed graph of steps with conditional routing. Each step
/// specifies what to execute, how (executor type), and where to route on success/failure.
/// Steps can loop back, branch on conditions, or terminate.
///
/// Example from llmboot:
/// ```text
/// propose → discuss (human_gate) → {
///   approve → integrate → implement → done
///   OR request_changes → revise → discuss (loop back)
/// }
/// ```
pub trait Workflow: Send + Sync {
    /// Workflow identifier (e.g., "adr-lifecycle-test-v1").
    fn id(&self) -> &str;

    /// Human-readable workflow description.
    fn description(&self) -> &str;

    /// Get a step by id; returns None if step doesn't exist.
    fn step(&self, id: &str) -> Option<&dyn WorkflowStep>;

    /// First step to execute (the workflow entry point).
    fn start_step_id(&self) -> &str;

    /// All step ids in this workflow (in definition order).
    fn step_ids(&self) -> Vec<&str>;

    /// Whether this workflow supports variable substitution (e.g., {{previous_step_output}}).
    fn supports_variable_substitution(&self) -> bool {
        true
    }
}
