use crate::output::Output;

use lightningcss::{
    stylesheet::{ParserFlags, ParserOptions, PrinterOptions, StyleSheet},
    targets::{Browsers, Targets},
};
use std::{
    ffi::OsStr,
    fs::{read_dir, File},
    io::Read,
    io::Write,
    path::{Path, PathBuf},
};

fn targets() -> Targets {
    Targets {
        browsers: Some(Browsers {
            safari: Some((13 << 16) | (2 << 8)),
            ..Browsers::default()
        }),
        ..Targets::default()
    }
}

fn printer_options<'a>() -> PrinterOptions<'a> {
    PrinterOptions {
        minify: false,
        targets: targets(),
        ..PrinterOptions::default()
    }
}

fn parser_options<'a>() -> ParserOptions<'a, 'a> {
    ParserOptions {
        flags: ParserFlags::NESTING,
        ..ParserOptions::default()
    }
}

#[derive(Debug)]
pub struct Asset {
    path: PathBuf,
}

impl Asset {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    #[must_use]
    pub fn file_name(&self) -> &OsStr {
        self.path.file_name().unwrap()
    }

    #[must_use]
    fn transform(&self) -> String {
        let mut file = File::open(&self.path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let stylesheet = StyleSheet::parse(&data, parser_options()).unwrap();
        stylesheet.to_css(printer_options()).unwrap().code
    }

    pub fn from_dir(path_str: &str) -> std::io::Result<Vec<Self>> {
        let path = Path::new(path_str);
        assert!(path.is_dir());
        Ok(read_dir(path)?
            .filter_map(|e| {
                e.map_or(None, |f| match f.file_type() {
                    Ok(file_type) if file_type.is_file() => Some(Self::new(f.path())),
                    _ => None,
                })
            })
            .collect())
    }
}

pub fn copy() {
    for asset in Asset::from_dir(crate::ASSETS).unwrap() {
        Output::asset(asset.file_name())
            .write_all(asset.transform().as_bytes())
            .unwrap();
    }
}
