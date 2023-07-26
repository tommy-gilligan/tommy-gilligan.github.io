use serde::Deserialize;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use syntect::html::{ClassedHTMLGenerator, ClassStyle};

mod markdown_options;
use markdown_options::MARKDOWN_OPTIONS;

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Frontmatter {
    pub title: String,
}

markup::define! {
    Layout<'a>(content: &'a str, frontmatter: Frontmatter) {
        @markup::doctype()
        html {
            head {
                title { @frontmatter.title }
                style { @include_str!("layout.css") }
            }
            body {
                header { h1 { @frontmatter.title } }
                #main { @markup::raw(content) }
                footer { "(c) " 2023 }
            }
        }
    }
}

fn format_code(code: &str) -> String {
    let binding = SyntaxSet::load_defaults_newlines();
    let syntax = binding.find_syntax_by_extension("rs").unwrap();
    let ss = SyntaxSet::load_defaults_newlines();
    let mut html_generator = ClassedHTMLGenerator::new_with_class_style(syntax, &ss, ClassStyle::Spaced);
    for line in LinesWithEndings::from(code) {
        html_generator.parse_html_for_line_which_includes_newline(line).unwrap();
    }
    html_generator.finalize()
}

fn find_code(mdast: markdown::mdast::Node) {
    println!("{}", format_code("pub struct Wow { hi: u64 }\nfn blah() -> u64 {}"));
}

fn frontmatter(contents: &str) -> Frontmatter {
    if let markdown::mdast::Node::Yaml(markdown::mdast::Yaml { value, .. }) =
        &markdown::to_mdast(contents, &MARKDOWN_OPTIONS.parse)
            .unwrap()
            .children()
            .unwrap()[0]
    {
        return serde_yaml::from_str(value).unwrap();
    } else if let markdown::mdast::Node::Toml(markdown::mdast::Toml { value, .. }) =
        &markdown::to_mdast(contents, &MARKDOWN_OPTIONS.parse)
            .unwrap()
            .children()
            .unwrap()[0]
    {
        return toml::from_str(value).unwrap();
    }
    panic!()
}

pub fn render(contents: &str) -> String {
    let mdast = markdown::to_mdast(contents, &MARKDOWN_OPTIONS.parse).unwrap();
    for child in (&mdast.children()).unwrap() {
        match child {
            markdown::mdast::Node::Code(markdown::mdast::Code { value, lang, .. }) => {
                println!("{:?}", format_code(value))
            },
            a => println!("{:?}", a)
        }
    }
    Layout {
        frontmatter: Frontmatter { title: "".to_string() },
        content: &markdown::to_html_with_options(contents, &MARKDOWN_OPTIONS).unwrap(),
    }
    .to_string()
}
