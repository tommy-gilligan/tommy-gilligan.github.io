use crate::{frontmatter::Frontmatter, layout::Layout, output::Output, view::CodeContainer};
use chrono::{DateTime, TimeZone, Utc};
use regex::Regex;

use askama::Template;
use std::{
    ffi::OsStr,
    fs::{read_dir, File},
    io::Write,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

const EXTENSION: &str = "html";

#[derive(Debug)]
pub struct Article {
    path: PathBuf,
}

pub fn replace_code(contents: &mut String) {
    // BROKEN: multibyte characters
    let mdast = markdown::to_mdast(contents, &crate::markdown::OPTIONS.parse).unwrap();

    let mut file_offset_accum: isize = 0;
    for child in (mdast.children()).unwrap() {
        if let markdown::mdast::Node::Code(markdown::mdast::Code {
            value,
            lang,
            position,
            ..
        }) = child
        {
            let code = CodeContainer {
                formatted_code: value,
                language: lang.as_ref().unwrap(),
            }
            .to_string();
            let start = position.clone().unwrap().start.offset;
            let end = position.clone().unwrap().end.offset;

            contents.replace_range(
                usize::try_from(isize::try_from(start).unwrap() + file_offset_accum).unwrap()
                    ..usize::try_from(isize::try_from(end).unwrap() + file_offset_accum).unwrap(),
                &code,
            );
            file_offset_accum += isize::try_from(code.len()).unwrap()
                - (isize::try_from(end).unwrap() - isize::try_from(start).unwrap());
        }
    }
}

// fn highlight(code: &str, language: &str) -> String {
//     match language {
//         "rust" => String::from_utf8(crate::syntax_highlighting::highlight(
//             code.as_bytes(),
//             crate::syntax_highlighting::Language::Rust,
//         ))
//         .unwrap(),
//         "python" => String::from_utf8(crate::syntax_highlighting::highlight(
//             code.as_bytes(),
//             crate::syntax_highlighting::Language::Python,
//         ))
//         .unwrap(),
//         "bash" | "zsh" | "sh" => String::from_utf8(crate::syntax_highlighting::highlight(
//             code.as_bytes(),
//             crate::syntax_highlighting::Language::Sh,
//         ))
//         .unwrap(),
//         _ => String::new(),
//     }
// }

impl Article {
    fn new(path: PathBuf) -> Self {
        let input_file = File::open(&path).unwrap();
        let mut buf_reader = BufReader::new(input_file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        Self { path }
    }

    #[must_use]
    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        // let updated_at = Utc
        //     .timestamp_opt(self.repo.latest(&self.path).time().seconds(), 0)
        //     .unwrap();
        // if updated_at > self.published_at() {
        //     Some(updated_at)
        // } else {
        //     None
        // }
        None
    }

    #[must_use]
    pub fn published_at(&self) -> DateTime<Utc> {
        self.frontmatter()
            .published_at
            .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap())
    }

    #[must_use]
    pub fn file_stem(&self) -> &OsStr {
        self.path.file_stem().unwrap()
    }

    #[must_use]
    pub fn title(&self) -> String {
        self.frontmatter().title
    }

    #[must_use]
    pub fn description(&self) -> String {
        self.frontmatter().description
    }

    #[must_use]
    pub fn output_path(&self, dir: &Path, extension: &str) -> PathBuf {
        let mut path = dir.join(self.path.file_name().unwrap());
        path.set_extension(extension);
        path
    }

    #[must_use]
    fn is_published(&self) -> bool {
        matches!(self.frontmatter().published, Some(true))
    }

    #[must_use]
    fn contents(&self) -> String {
        let input_file = File::open(&self.path).unwrap();
        let mut buf_reader = BufReader::new(input_file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        contents
    }

    fn frontmatter(&self) -> Frontmatter {
        crate::frontmatter::frontmatter(&self.contents())
    }

    pub fn from_dir(path_str: &str) -> std::io::Result<Vec<Self>> {
        let path = Path::new(path_str);
        assert!(path.is_dir());
        Ok(read_dir(path)?
            .filter_map(|e| {
                e.map_or(None, |f| match f.file_type() {
                    Ok(file_type) if file_type.is_file() => match f.path().extension() {
                        Some(extension) if extension == OsStr::new(EXTENSION) => {
                            let article = Self::new(f.path());
                            if article.is_published() {
                                Some(article)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    _ => None,
                })
            })
            .collect())
    }

    pub fn highlight(&mut self) -> String {
        let binding = self.contents();
        let dom = tl::parse(&binding, tl::ParserOptions::default()).unwrap();

        // let code_handle = dom.query_selector("code[data-language]").unwrap().next().unwrap();

        // let parser = dom.parser();
        // let code = code_handle.get(parser)
        //   .expect("Failed to resolve node")
        //   .as_tag()
        //   .expect("Failed to cast Node to HTMLTag");

        // let language = code.attributes().get("data-language").unwrap().unwrap().try_as_utf8_str().unwrap();
        // let highlighted = highlight(&code.inner_text(parser), language);

        // let mut parser = dom.parser_mut();
        // let code_tag = code_handle.get_mut(parser).unwrap().as_tag_mut().unwrap();
        // let mut highlighted_node = tl::parse(&highlighted, tl::ParserOptions::default()).unwrap();

        // let mut children_mut = code_tag.children_mut();
        // children_mut = highlighted_node.children_mut();

        dom.outer_html()
    }
}

pub fn watch<W>(watcher: &mut W)
where
    W: notify::Watcher,
{
    watcher
        .watch(Path::new(crate::ARTICLES), notify::RecursiveMode::Recursive)
        .unwrap();
}

pub fn render() {
    for mut article in Article::from_dir(crate::ARTICLES).unwrap() {
        let re = Regex::new(r"</br>|</img>|</hr>").unwrap();

        let layed_out = Layout {
            description: &article.description(),
            body: &re.replace_all(&article.highlight(), ""),
            lang: &crate::locale::language_tag(),
            sitemap: "sitemap",
            title: crate::TITLE,
            page_title: Some(&article.title()),
        }
        .render()
        .unwrap();

        Output::page(article.file_stem())
            .write_all(layed_out.as_bytes())
            .unwrap();
    }
}
