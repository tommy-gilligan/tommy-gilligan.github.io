use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use syntect::html::{ClassedHTMLGenerator, ClassStyle};

pub fn format_code(code: &str) -> String {
    let binding = SyntaxSet::load_defaults_newlines();
    let syntax = binding.find_syntax_by_extension("zsh").unwrap();
    let ss = SyntaxSet::load_defaults_newlines();
    let mut html_generator = ClassedHTMLGenerator::new_with_class_style(syntax, &ss, ClassStyle::Spaced);
    for line in LinesWithEndings::from(code) {
        html_generator.parse_html_for_line_which_includes_newline(line).unwrap();
    }
    html_generator.finalize()
}
