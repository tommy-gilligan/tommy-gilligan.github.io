use lightningcss::{
    stylesheet::{MinifyOptions, ParserFlags, ParserOptions, PrinterOptions, StyleSheet},
    targets::{Browsers, Targets},
};
use std::io::Read;
use std::{
    fs::File,
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
        minify: true,
        targets: targets(),
        ..PrinterOptions::default()
    }
}

fn minify_options() -> MinifyOptions {
    MinifyOptions {
        targets: targets(),
        ..MinifyOptions::default()
    }
}

fn parser_options<'a>() -> ParserOptions<'a, 'a> {
    ParserOptions {
        flags: ParserFlags::NESTING,
        ..ParserOptions::default()
    }
}

#[derive(Clone)]
pub struct Style {
    path: PathBuf,
}

impl Style {
    #[must_use]
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    #[must_use]
    pub fn style(&self) -> String {
        let mut file = File::open(&self.path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let mut stylesheet = StyleSheet::parse(&data, parser_options()).unwrap();
        stylesheet.minify(minify_options()).unwrap();
        stylesheet.to_css(printer_options()).unwrap().code
    }
}
