use std::{
    ffi::OsStr,
    fs::{read_dir, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

mod frontmatter;
mod markdown_options;
pub use crate::page::frontmatter::Frontmatter;
use chrono::{DateTime, TimeZone, Utc};
use git2::Repository;

const EXTENSION: &str = "md";

#[derive(Debug)]
pub struct Page {
    path: PathBuf,
}

markup::define! {
    CodeContainer<'a>(
        formatted_code: &'a str,
        language: &'a str,
    ) {
        pre {
            code.{format!("language-{}", language)} {
                @markup::raw(formatted_code)
            }
        }
    }
}

pub fn replace_code(contents: &mut String) {
    // fix mixing bytes with chars
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

impl Page {
    const fn new(path: PathBuf) -> Self {
        Self { path }
    }

    #[must_use]
    pub fn published_at(&self) -> DateTime<Utc> {
        self.history().first().unwrap().0
    }

    #[must_use]
    pub fn file_stem(&self) -> &OsStr {
        self.path.file_stem().unwrap()
    }

    #[must_use]
    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        let updated_at = self.history().last().unwrap().0;
        if updated_at == self.published_at() {
            None
        } else {
            Some(updated_at)
        }
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
    pub fn history(&self) -> Vec<(DateTime<Utc>, String, String, String)> {
        let mut path = self.path.clone();
        let mut res = Vec::new();
        path = path.strip_prefix("./").unwrap().to_path_buf();
        let repo = match Repository::open(".") {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {e}"),
        };
        let mut tree = repo.revwalk().unwrap();
        tree.push_head().unwrap();
        for oid in tree {
            let o = oid.unwrap();
            let c = repo.find_commit(o).unwrap();
            if c.tree().unwrap().get_path(&path).is_ok() {
                res.push((
                    Utc.timestamp_opt(c.time().seconds(), 0).unwrap(),
                    c.message().unwrap().to_owned(),
                    String::new(),
                    c.id().to_string(),
                ));
            }
        }
        res
    }

    #[must_use]
    pub fn output_path(&self, dir: &Path, extension: &str) -> PathBuf {
        let mut path = dir.clone().join(self.path.file_name().unwrap());
        path.set_extension(extension);
        path
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

    // probably should exclude index.md
    pub fn from_dir(path_str: &str) -> std::io::Result<Vec<Self>> {
        let path = Path::new(path_str);
        assert!(path.is_dir());
        Ok(read_dir(path)?
            .filter_map(|e| {
                e.map_or(None, |f| match f.file_type() {
                    Ok(file_type) if file_type.is_file() => match f.path().extension() {
                        Some(extension) if extension == OsStr::new(EXTENSION) => {
                            Some(Self::new(f.path()))
                        }
                        _ => None,
                    },
                    _ => None,
                })
            })
            .collect())
    }
}
