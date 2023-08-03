use lightningcss::{
    stylesheet::{MinifyOptions, ParserFlags, ParserOptions, PrinterOptions, StyleSheet},
    targets::{Browsers, Targets},
};

pub fn style() -> String {
    let mut stylesheet = StyleSheet::parse(
        include_str!("style.css"),
        ParserOptions {
            flags: ParserFlags::NESTING,
            ..ParserOptions::default()
        },
    )
    .unwrap();

    let browsers = Browsers {
        safari: Some((13 << 16) | (2 << 8)),
        ..Browsers::default()
    };

    stylesheet
        .minify(MinifyOptions {
            targets: Targets {
                browsers: Some(browsers),
                ..Targets::default()
            },
            ..MinifyOptions::default()
        })
        .unwrap();

    let res = stylesheet
        .to_css(PrinterOptions {
            minify: true,
            targets: Targets {
                browsers: Some(browsers),
                ..Targets::default()
            },
            ..PrinterOptions::default()
        })
        .unwrap();
    res.code
}
