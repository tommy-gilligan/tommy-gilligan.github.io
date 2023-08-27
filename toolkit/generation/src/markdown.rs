mod options;

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

    #[must_use]
    pub fn render(&self) -> String {
        markdown::to_html_with_options(&self.source, &options::OPTIONS).unwrap()
    }
}
