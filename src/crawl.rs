use sitemap::structs::UrlEntry;
use sitemap::writer::SiteMapWriter;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::fs::read_dir;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

const EXTENSION: &str = "html";
#[allow(dead_code)]
mod config;

pub fn from_dir(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    Ok(read_dir(path)?
        .filter_map(|e| {
            e.map_or(None, |f| match f.file_type() {
                Ok(file_type) if file_type.is_file() => match f.path().extension() {
                    Some(extension) if extension == OsStr::new(EXTENSION) => Some(f.path()),
                    _ => None,
                },
                _ => None,
            })
        })
        .collect())
}

fn main() {
    let base_url = config::base_url();
    println!("{:?}", base_url);

    let output_dir = Path::new("./_site");
    create_dir_all(output_dir).unwrap();

    let mut output = File::create(output_dir.clone().join("sitemap.xml")).unwrap();
    let sitemap_writer = SiteMapWriter::new(&mut output);
    let mut urlwriter = sitemap_writer
        .start_urlset()
        .expect("Unable to write urlset");

    for file in from_dir(output_dir).unwrap() {
        let mut url = base_url.clone();
        url.set_path(file.file_name().unwrap().to_str().unwrap());
        urlwriter
            .url(UrlEntry::builder().loc(url.to_string()))
            .expect("Unable to write url");
        println!("{:?}", file);
    }

    urlwriter.end().expect("Unable to write close tags");
}
