use lightningcss::{
    stylesheet::{ParserFlags, ParserOptions, PrinterOptions, StyleSheet},
    targets::{Browsers, Targets},
};
use std::io::Read;
use std::{fs::File, path::Path};

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

#[must_use]
pub fn style() -> String {
    let mut file = File::open(Path::new(crate::STYLE)).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let stylesheet = StyleSheet::parse(&data, parser_options()).unwrap();
    stylesheet.to_css(printer_options()).unwrap().code
}
