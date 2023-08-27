// use syntect::html::{ClassStyle, ClassedHTMLGenerator};
// use syntect::parsing::SyntaxSet;
// use syntect::util::LinesWithEndings;
//
// pub fn format_code(code: &str, language: &str) -> String {
//     let binding = SyntaxSet::load_defaults_newlines();
//     let syntax = binding
//         .find_syntax_by_extension(language)
//         .unwrap_or_else(|| binding.find_syntax_plain_text());
//     let ss = SyntaxSet::load_defaults_newlines();
//     let mut html_generator =
//         ClassedHTMLGenerator::new_with_class_style(syntax, &ss, ClassStyle::Spaced);
//     for line in LinesWithEndings::from(code) {
//         html_generator
//             .parse_html_for_line_which_includes_newline(line)
//             .unwrap();
//     }
//     html_generator.finalize()
// }
