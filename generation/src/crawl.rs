use std::{ffi::OsStr, fs::read_dir, path::Path};
use url::Url;

pub struct Crawler {
    urls: Vec<Url>,
}

impl Crawler {
    pub fn new() -> Self {
        let urls: Vec<Url> = read_dir(Path::new("./_site"))
            .unwrap()
            .filter_map(|e| {
                e.map_or(None, |f| match f.file_type() {
                    Ok(file_type) if file_type.is_file() => match f.path().extension() {
                        Some(extension) if extension == OsStr::new("html") => Some(
                            format!(
                                "https://example.com/{}",
                                f.path().file_name().unwrap().to_str().unwrap()
                            )
                            .parse()
                            .unwrap(),
                        ),
                        _ => None,
                    },
                    _ => None,
                })
            })
            .collect();
        Self { urls }
    }
}

impl Iterator for Crawler {
    type Item = Url;

    fn next(&mut self) -> Option<Url> {
        self.urls.pop()
    }
}
