use crate::view::CodeContainer;
use markdown::{
    mdast::{Code, Node},
    CompileOptions, Constructs, LineEnding, Options, ParseOptions,
};

pub const OPTIONS: Options = Options {
    compile: CompileOptions {
        allow_dangerous_html: true,
        allow_dangerous_protocol: false,
        default_line_ending: LineEnding::LineFeed,
        gfm_footnote_back_label: None,
        gfm_footnote_clobber_prefix: None,
        gfm_footnote_label: None,
        gfm_footnote_label_attributes: None,
        gfm_footnote_label_tag_name: None,
        gfm_tagfilter: false,
    },
    parse: ParseOptions {
        constructs: Constructs {
            attention: true,
            gfm_autolink_literal: true,
            autolink: false,
            block_quote: true,
            character_escape: true,
            character_reference: false,
            code_indented: true,
            code_fenced: true,
            code_text: true,
            definition: true,
            frontmatter: true,
            gfm_footnote_definition: true,
            gfm_label_start_footnote: true,
            gfm_strikethrough: true,
            gfm_table: true,
            gfm_task_list_item: false,
            hard_break_escape: false,
            hard_break_trailing: false,
            html_text: true,
            html_flow: true,
            heading_setext: false,
            heading_atx: true,
            label_start_image: true,
            label_start_link: true,
            label_end: true,
            list_item: true,
            math_flow: true,
            math_text: true,
            mdx_esm: false,
            mdx_expression_flow: false,
            mdx_expression_text: false,
            mdx_jsx_flow: false,
            mdx_jsx_text: false,
            thematic_break: true,
        },
        gfm_strikethrough_single_tilde: true,
        math_text_single_dollar: true,
        mdx_expression_parse: None,
        mdx_esm_parse: None,
    },
};

pub struct Markdown {
    source: String,
}

impl Markdown {
    #[must_use]
    pub const fn new(source: String) -> Self {
        Self { source }
    }

    pub fn replace<F>(&mut self, substitution: F)
    where
        F: Fn(&markdown::mdast::Node) -> Option<String>,
    {
        // fix mixing bytes with chars
        let mdast = markdown::to_mdast(&self.source, &markdown::ParseOptions::default()).unwrap();
        let mut file_offset_accum: isize = 0;

        let empty = Vec::new();
        let iter = traversal::dft_pre(&mdast, |node: &markdown::mdast::Node| {
            node.children().unwrap_or(&empty).iter()
        });

        for (_, node) in iter {
            if let Some(code) = substitution(node) {
                let position = node.position();
                let start = position.unwrap().start.offset;
                let end = position.unwrap().end.offset;
                let range = usize::try_from(isize::try_from(start).unwrap() + file_offset_accum)
                    .unwrap()
                    ..usize::try_from(isize::try_from(end).unwrap() + file_offset_accum).unwrap();

                self.source.replace_range(range, &code);
                file_offset_accum += isize::try_from(code.len()).unwrap()
                    - (isize::try_from(end).unwrap() - isize::try_from(start).unwrap());
            }
        }
    }

    pub fn highlight(&mut self) {
        self.replace(|node| {
            if let Node::Code(Code { value, lang, .. }) = node {
                lang.as_ref().map_or_else(
                    || Some(value.clone()),
                    |s| {
                        let code = match s.as_str() {
                            "rust" => String::from_utf8(crate::syntax_highlighting::highlight(
                                value.as_bytes(),
                                crate::syntax_highlighting::Language::Rust,
                            ))
                            .unwrap(),
                            "python" => String::from_utf8(crate::syntax_highlighting::highlight(
                                value.as_bytes(),
                                crate::syntax_highlighting::Language::Python,
                            ))
                            .unwrap(),
                            "bash" | "zsh" | "sh" => {
                                String::from_utf8(crate::syntax_highlighting::highlight(
                                    value.as_bytes(),
                                    crate::syntax_highlighting::Language::Sh,
                                ))
                                .unwrap()
                            }
                            _ => String::new(),
                        };
                        Some(
                            CodeContainer {
                                formatted_code: &code,
                                language: s,
                            }
                            .to_string(),
                        )
                    },
                )
            } else {
                None
            }
        });
    }

    #[must_use]
    pub fn render(&self) -> String {
        markdown::to_html_with_options(&self.source, &crate::markdown::OPTIONS).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::markdown::Markdown;
    use indoc::indoc;

    #[test]
    fn test_highlight_rust() {
        let mut m = Markdown::new(
            indoc! {r#"
                # Some Code
                ```rust
                const X: u8 = 123;
                ```

                1. first it takes the number
                2. then it doubles it
                3. and then it returns it
            "#}
            .to_owned(),
        );
        m.highlight();

        assert_eq!(
            m.render(),
            indoc! { r#"<h1>Some Code</h1>
            <pre><code class="language-rust"><span class="highlight-10">const</span> <span class="highlight-4">X</span>: <span class="highlight-17">u8</span> = <span class="highlight-3">123</span>;</code></pre>
            <ol>
            <li>first it takes the number</li>
            <li>then it doubles it</li>
            <li>and then it returns it</li>
            </ol>
            "# }
        );
    }
}
