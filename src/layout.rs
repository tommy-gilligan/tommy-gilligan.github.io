use serde::Deserialize;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use syntect::html::{ClassedHTMLGenerator, ClassStyle};

mod markdown_options;
use markdown_options::MARKDOWN_OPTIONS;
use std::process::{Command, Stdio};
use std::io::{Write, Read};

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

const MAIN: &str = r#"
    fn main() {
        println!("{:?}", double(2));
   }
"#;

fn run_code(code: String) -> String {
    let mut child = Command::new("rustc")
        .stdin(Stdio::piped())
        .args([ "-", "--crate-type", "bin", "-o", "mycode"])
        .spawn()
        .expect("failed to execute process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let computation = std::thread::spawn(move || {
        stdin.write(MAIN.as_bytes()).unwrap();
        stdin.write_all(
            code.clone().as_bytes()
        ).expect("Failed to write to stdin");
    });
    child.wait().unwrap();
    std::str::from_utf8(
        &Command::new("./mycode").output().unwrap().stdout
    ).unwrap().to_owned()
}

fn format_code(code: &str) -> String {
    let mut child = Command::new("rustc")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(
            [
                "-",
                "--crate-type",
                "cdylib",
                "--target",
                "wasm32-unknown-unknown",
                "-C",
                "opt-level=z",
                "-C",
                "lto=true",
            ]
        )
        .spawn()
        .expect("failed to execute process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(code.as_bytes()).expect("Failed to write to stdin");

    let mut stderr = child.stderr.take().expect("Failed to open stderr");
    let mut buf = String::new();
    let computation = std::thread::spawn(move || {
        std::io::read_to_string(stderr).unwrap()
    });

    let binding = SyntaxSet::load_defaults_newlines();
    let syntax = binding.find_syntax_by_extension("rs").unwrap();
    let ss = SyntaxSet::load_defaults_newlines();
    let mut html_generator = ClassedHTMLGenerator::new_with_class_style(syntax, &ss, ClassStyle::Spaced);
    for line in LinesWithEndings::from(code) {
        html_generator.parse_html_for_line_which_includes_newline(line).unwrap();
    }
    format!("{}<br>\nOutput:<br>\n<pre><code>{}</code></pre>", html_generator.finalize(), run_code(code.to_string()))
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
                return (Layout {
                    frontmatter: Frontmatter { title: "".to_string() },
                    content: &format_code(value),
                }
                .to_string());
            },
            _ => ()
        }
    }
    unreachable!()
}
