use std::{
    ffi::OsStr,
    fs::{read_dir, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

mod frontmatter;
mod markdown_options;
pub use crate::article::frontmatter::Frontmatter;

use crate::git::Git;
use crate::view::CodeContainer;
use chrono::{DateTime, TimeZone, Utc};
use git2::Commit;
use url::Url;

const EXTENSION: &str = "md";

#[derive(Debug)]
pub struct Article {
    path: PathBuf,
    repo: Git,
}

pub fn replace_code(contents: &mut String) {
    // BROKEN: multibyte characters
    let mdast = markdown::to_mdast(contents, &markdown_options::MARKDOWN_OPTIONS.parse).unwrap();

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

impl Article {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            repo: Git::new(),
        }
    }

    #[must_use]
    pub fn link_urls(&self) -> Vec<Url> {
        let mdast = markdown::to_mdast(&self.contents(), &markdown_options::MARKDOWN_OPTIONS.parse)
            .unwrap();

        (mdast.children())
            .unwrap()
            .iter()
            .filter_map(|child| {
                if let markdown::mdast::Node::Paragraph(markdown::mdast::Paragraph {
                    children,
                    ..
                }) = child
                {
                    Some(children.iter().filter_map(|child| {
                        if let markdown::mdast::Node::Link(markdown::mdast::Link { url, .. }) =
                            child
                        {
                            Some(url.parse().unwrap())
                        } else {
                            None
                        }
                    }))
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    #[must_use]
    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        let updated_at = self
            .history()
            .into_iter()
            .map(|commit| Utc.timestamp_opt(commit.time().seconds(), 0).unwrap())
            .max()
            .unwrap();
        if updated_at > self.published_at() {
            Some(updated_at)
        } else {
            None
        }
    }

    #[must_use]
    pub fn published_at(&self) -> DateTime<Utc> {
        self.frontmatter().published_at.unwrap_or_else(|| {
            self.history()
                .into_iter()
                .map(|commit| Utc.timestamp_opt(commit.time().seconds(), 0).unwrap())
                .min()
                .unwrap()
        })
    }

    #[must_use]
    pub fn truncated_history(&self) -> Vec<Commit> {
        let truncated: Vec<_> = self
            .history()
            .into_iter()
            .filter(|commit| {
                Utc.timestamp_opt(commit.time().seconds(), 0).unwrap() >= self.published_at()
            })
            .collect();

        if truncated.is_empty() {
            vec![self.history().into_iter().last().unwrap()]
        } else {
            truncated
        }
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
    pub fn history(&self) -> Vec<Commit> {
        self.repo.commits_for(&self.path)
    }

    #[must_use]
    pub fn output_path(&self, dir: &Path, extension: &str) -> PathBuf {
        let mut path = dir.join(self.path.file_name().unwrap());
        path.set_extension(extension);
        path
    }

    #[must_use]
    pub fn is_published(&self) -> bool {
        matches!(self.frontmatter().published, Some(true))
    }

    #[must_use]
    pub fn contents(&self) -> String {
        let input_file = File::open(&self.path).unwrap();
        let mut buf_reader = BufReader::new(input_file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        contents
    }

    pub fn body(&mut self) -> String {
        let mut contents = self.contents();
        replace_code(&mut contents);
        markdown::to_html_with_options(&contents, &markdown_options::MARKDOWN_OPTIONS).unwrap()
    }

    fn frontmatter(&self) -> Frontmatter {
        frontmatter::frontmatter(&self.contents(), &markdown_options::MARKDOWN_OPTIONS.parse)
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
}
