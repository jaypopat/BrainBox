use comrak::{parse_document, Arena, ComrakOptions, NodeValue};

fn parse_markdown(content: &str) -> Vec<String> {
    let arena = Arena::new();
    let root = parse_document(&arena, content, &ComrakOptions::default());
    let mut results = Vec::new();

    // Traversing the AST
    for node in root.descendants() {
        match &node.data.borrow().value {
            NodeValue::Text(text) => {
                results.push(format!("Text: {}", text));
            }
            NodeValue::Heading(level) => {
                results.push(format!("Heading Level {}: {}", level, text));
            }
            NodeValue::List(_) => {
                results.push("List Item".to_string());
            }
            NodeValue::CodeBlock(info) => {
                results.push(format!("Code Block (info: {})", info));
            }
            NodeValue::HtmlBlock(_) => {
                results.push("HTML Block".to_string());
            }
            NodeValue::Paragraph => {
                results.push("Paragraph".to_string());
            }
            NodeValue::BlockQuote => {
                results.push("Block Quote".to_string());
            }
            NodeValue::Item => {
                results.push("List Item".to_string());
            }
            NodeValue::Code(_) => {
                results.push("Code".to_string());
            }
            NodeValue::HtmlInline => {
                results.push("HTML Inline".to_string());
            }
            _ => {}
        }
    }
    results
}
