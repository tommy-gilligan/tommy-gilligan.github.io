use lightningcss::{
    stylesheet::{ParserFlags, ParserOptions, PrinterOptions, StyleSheet},
    targets::{Browsers, Targets},
};
use std::{fs::File, io::Read, path::Path};

pub const EXTENSION: &str = "css";

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

pub fn transform(path: &Path) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let stylesheet = StyleSheet::parse(&data, parser_options()).unwrap();
    let css = stylesheet.to_css(printer_options()).unwrap();
    css.code.into_bytes()
}
