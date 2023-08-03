use std::{
    ffi::OsStr,
    fs::{read_dir, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

mod frontmatter;
mod markdown_options;
pub use crate::source_file::frontmatter::Frontmatter;
use crate::syntax_highlighting::format_code;

const EXTENSION: &str = "md";

pub struct SourceFile {
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
                formatted_code: &format_code(value, lang.as_ref().unwrap()),
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

impl SourceFile {
    const fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn output_path(&self, dir: &Path, extension: &str) -> PathBuf {
        let mut path = dir.clone().join(&self.path.file_name().unwrap());
        path.set_extension(extension);
        path
    }

    pub fn contents(&self) -> String {
        let input_file = File::open(&self.path).unwrap();
        let mut buf_reader = BufReader::new(input_file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        contents
    }

    pub fn frontmatter(&self) -> Frontmatter {
        frontmatter::frontmatter(&self.contents(), &markdown_options::MARKDOWN_OPTIONS.parse)
    }

    pub fn body(&mut self) -> String {
        let mut contents = self.contents();
        replace_code(&mut contents);
        markdown::to_html_with_options(&contents, &markdown_options::MARKDOWN_OPTIONS).unwrap()
    }

    // probably should exclude index.md
    pub fn from_dir(path: &Path) -> std::io::Result<Vec<Self>> {
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
