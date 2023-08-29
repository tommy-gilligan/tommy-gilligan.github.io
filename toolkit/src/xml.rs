use libxml::parser::{Parser, ParserOptions};
use libxml::schemas::{SchemaParserContext, SchemaValidationContext};

// RSS does not have an official 'schema'

#[derive(Debug, PartialEq, Eq)]
pub enum MyResult {
    Ok,
    Xml,
    Xsd,
    XmlXsd,
}

#[must_use]
pub fn validate(xml: &[u8], xsd: Option<&[u8]>) -> MyResult {
    let options = ParserOptions {
        recover: false,
        no_def_dtd: false,
        no_error: true,
        no_warning: true,
        pedantic: true,
        ..ParserOptions::default()
    };
    Parser::default()
        .parse_string_with_options(xml, options)
        .map_or(MyResult::Xml, |xml| {
            let options = ParserOptions {
                recover: false,
                no_def_dtd: false,
                no_error: true,
                no_warning: true,
                pedantic: true,
                ..ParserOptions::default()
            };
            xsd.map_or(MyResult::Ok, |xsd| {
                Parser::default()
                    .parse_string_with_options(xsd, options)
                    .map_or(MyResult::Xsd, |xsd| {
                        SchemaValidationContext::from_parser(
                            &mut SchemaParserContext::from_document(&xsd),
                        )
                        .map_or(MyResult::Xsd, |mut xsd| {
                            if xsd.validate_document(&xml).is_ok() {
                                MyResult::Ok
                            } else {
                                MyResult::XmlXsd
                            }
                        })
                    })
            })
        })
}

#[cfg(test)]
mod tests {
    const GOOD_SITEMAP: &[u8; 354] = br#"<?xml version="1.0" encoding="utf-8"?>
        <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
            <url>
                <loc>http://example.com/mysite</loc>
                <lastmod>2006-11-18</lastmod>
                <changefreq>daily</changefreq>
                <priority>0.8</priority>
            </url>
        </urlset>
    "#;

    const SITEMAP_MISSING_LOC: &[u8; 502] = br#"<?xml version="1.0" encoding="utf-8"?>
        <urlset xmlns="https://www.sitemaps.org/schemas/sitemap/0.9"
           xmlns:xsi="https://www.w3.org/2001/XMLSchema-instance"
           xsi:schemaLocation="https://www.sitemaps.org/schemas/sitemap/0.9 https://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd">
            <url>
                <lastmod>2006-11-18</lastmod>
                <changefreq>daily</changefreq>
                <priority>0.8</priority>
            </url>
        </urlset>
    "#;

    const SITEMAP_UNTERMINATED_URLSET: &[u8; 537] = br#"<?xml version="1.0" encoding="utf-8"?>
        <urlset xmlns="https://www.sitemaps.org/schemas/sitemap/0.9"
           xmlns:xsi="https://www.w3.org/2001/XMLSchema-instance"
           xsi:schemaLocation="https://www.sitemaps.org/schemas/sitemap/0.9 https://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd">
            <url>
                <loc>http://example.com/mysite</loc>
                <lastmod>2006-11-18</lastmod>
                <changefreq>daily</changefreq>
                <priority>0.8</priority>
            </url>
    "#;

    const SITEMAP_XSD: &[u8; 3728] = include_bytes!("sitemap.xsd");

    #[test]
    fn test_validate_good_sitemap() {
        assert_eq!(super::validate(GOOD_SITEMAP, None), super::MyResult::Ok);
    }

    #[test]
    fn test_validate_bad_sitemap() {
        assert_eq!(
            super::validate(SITEMAP_MISSING_LOC, None),
            super::MyResult::Ok
        );
    }

    #[test]
    fn test_validate_unterminated_sitemap() {
        assert_eq!(
            super::validate(SITEMAP_UNTERMINATED_URLSET, None),
            super::MyResult::Xml
        );
    }

    #[test]
    fn test_validate_good_sitemap_with_xsd() {
        assert_eq!(
            super::validate(GOOD_SITEMAP, Some(SITEMAP_XSD)),
            super::MyResult::Ok
        );
    }

    #[test]
    fn test_validate_bad_sitemap_with_xsd() {
        assert_eq!(
            super::validate(SITEMAP_MISSING_LOC, Some(SITEMAP_XSD)),
            super::MyResult::XmlXsd
        );
    }

    #[test]
    fn test_validate_bad_sitemap_without_closing_urlset() {
        assert_eq!(
            super::validate(SITEMAP_UNTERMINATED_URLSET, Some(SITEMAP_XSD)),
            super::MyResult::Xml
        );
    }

    #[test]
    fn test_validate_bad_xsd() {
        assert_eq!(
            super::validate(GOOD_SITEMAP, Some(b"badxsd")),
            super::MyResult::Xsd
        );
    }
}
