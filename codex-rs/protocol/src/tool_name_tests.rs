use pretty_assertions::assert_eq;

use super::*;

#[test]
fn canonical_flat_name_preserves_plain_and_default_namespace_names() {
    assert_eq!(ToolName::plain("lookup").canonical_flat_name(), "lookup");
    assert_eq!(
        ToolName::namespaced(DEFAULT_FUNCTION_NAMESPACE, "lookup").canonical_flat_name(),
        "lookup"
    );
}

#[test]
fn canonical_flat_name_inserts_one_namespace_delimiter() {
    assert_eq!(
        ToolName::namespaced("mcp__sage", "sage_turn").canonical_flat_name(),
        "mcp__sage__sage_turn"
    );
    assert_eq!(
        ToolName::namespaced("mcp__sage__", "__sage_turn").canonical_flat_name(),
        "mcp__sage__sage_turn"
    );
}
