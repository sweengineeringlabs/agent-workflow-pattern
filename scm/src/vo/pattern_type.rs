//! The reasoning/interaction pattern a conversation loop follows.

/// Which conversation-loop pattern an agent follows, mirroring `llmboot`'s own
/// `agent_registry::AgentPatternType` (`features/agents/registry/src/spi/types.rs`) --
/// the reference implementation this ecosystem's real conversation-loop engines are ported
/// from. Not every variant has a real implementor in this org yet; each variant's own doc
/// comment says so honestly rather than implying otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    /// Reasoning + Acting loop: request a completion, dispatch any requested tool calls,
    /// feed results back, repeat until the model stops requesting tools. Real implementor:
    /// `edge-llm/agents/svc::ReactConversationLoop`.
    React,
    /// Chain-of-Thought prompting: the model reasons step by step in its own output before
    /// answering, with no tool dispatch loop. No real implementor in this org yet.
    CoT,
    /// Conversational multi-turn: plain back-and-forth exchange with no tool dispatch or
    /// planning structure. No real implementor in this org yet.
    Chat,
    /// Inline code-completion copilot: suggests completions for an existing artifact rather
    /// than running a full conversational loop. No real implementor in this org yet.
    Copilot,
    /// Plan then execute steps: generate a plan, execute its steps in dependency order,
    /// replan around a failed step when configured to. Real implementor:
    /// `edge-llm/agents/svc::PlanExecuteConversationLoop`.
    PlanExecute,
    /// Self-reflective iteration: draft a response, critique it, revise before returning it.
    /// Real implementor: `edge-llm/agents/svc::ReflexionConversationLoop`.
    Reflexion,
    /// Delegates to sub-agents rather than acting directly. No real implementor in this org
    /// yet -- the closest real analog is `agent-workflow-svc`'s own `Workflow`/`WorkflowStep`
    /// graph, which this crate declares the contract for.
    Orchestrator,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: PatternType
    #[test]
    fn test_pattern_type_all_seven_variants_construct_happy() {
        let variants = [
            PatternType::React,
            PatternType::CoT,
            PatternType::Chat,
            PatternType::Copilot,
            PatternType::PlanExecute,
            PatternType::Reflexion,
            PatternType::Orchestrator,
        ];
        assert_eq!(variants.len(), 7, "exactly seven variants, matching llmboot's source enum");
    }

    /// @covers: PatternType (PartialEq)
    #[test]
    fn test_pattern_type_equality_distinguishes_every_variant_happy() {
        let variants = [
            PatternType::React,
            PatternType::CoT,
            PatternType::Chat,
            PatternType::Copilot,
            PatternType::PlanExecute,
            PatternType::Reflexion,
            PatternType::Orchestrator,
        ];
        for (i, a) in variants.iter().enumerate() {
            for (j, b) in variants.iter().enumerate() {
                assert_eq!(
                    a == b,
                    i == j,
                    "variant at index {i} must equal itself and no other variant (comparing against index {j})"
                );
            }
        }
    }

    /// @covers: PatternType (Copy)
    #[test]
    fn test_pattern_type_copy_does_not_move_the_original_happy() {
        let original = PatternType::React;
        let copied = original;
        // If `Copy` were missing, using `original` after this line would be a compile error
        // (a moved value) -- this line is the real assertion, not the equality check below.
        assert_eq!(original, copied);
    }

    /// @covers: PatternType (Debug)
    #[test]
    fn test_pattern_type_debug_output_names_the_variant_happy() {
        assert_eq!(format!("{:?}", PatternType::PlanExecute), "PlanExecute");
        assert_eq!(format!("{:?}", PatternType::Orchestrator), "Orchestrator");
    }
}
