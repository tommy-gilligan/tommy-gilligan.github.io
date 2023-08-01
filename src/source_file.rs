use std::{
    ffi::OsStr,
    fs::{read_dir, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

mod frontmatter;
mod markdown_options;
pub use crate::source_file::frontmatter::Frontmatter;

const EXTENSION: &str = "md";

pub struct SourceFile {
    path: PathBuf,
}

impl SourceFile {
    const fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn output_path(&self, dir: &Path, extension: &str) -> PathBuf {
        let mut path = dir.clone().join(&self.path);
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

    pub fn body(&self) -> String {
        markdown::to_html_with_options(&self.contents(), &markdown_options::MARKDOWN_OPTIONS)
            .unwrap()
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
