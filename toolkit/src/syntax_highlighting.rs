use tree_sitter_highlight::HighlightEvent;
use tree_sitter_highlight::{HighlightConfiguration, Highlighter};

const HIGHLIGHT_NAMES: [&str; 21] = [
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "embedded",
    "escape",
    "function",
    "function.macro",
    "function.method",
    "keyword",
    "label",
    "number",
    "operator",
    "property",
    "string",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

#[derive(Clone, Copy)]
pub enum Language {
    Sh,
    Rust,
}

pub fn highlight(source: &[u8], lang: Language) -> Vec<u8> {
    let mut config = match lang {
        Language::Sh => HighlightConfiguration::new(
            tree_sitter_bash::language(),
            tree_sitter_bash::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap(),
        Language::Rust => HighlightConfiguration::new(
            tree_sitter_rust::language(),
            tree_sitter_rust::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap(),
    };
    config.configure(&HIGHLIGHT_NAMES);

    Highlighter::new()
        .highlight(&config, source, None, |_| None)
        .unwrap()
        .flat_map(|event| match event {
            Ok(HighlightEvent::HighlightStart(s)) => {
                Vec::from(format!("<span class=\"highlight-{}\">", s.0).as_bytes())
            }
            Ok(HighlightEvent::HighlightEnd) => Vec::from(&b"</span>"[..]),
            Ok(HighlightEvent::Source { start, end }) => Vec::from(&source[start..end]),
            _ => Vec::new(),
        })
        .collect()
}

#[test]
fn test_highlight() {
    assert_eq!(
        &highlight(b"const X: u8 = 123;", Language::Rust),
        br#"<span class="highlight-10">const</span> <span class="highlight-4">X</span>: <span class="highlight-17">u8</span> = <span class="highlight-3">123</span>;"#
    );
}
