#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use std::{
    fs::{create_dir_all, File},
    io::prelude::*,
    path::Path,
};

mod layout;
mod source_file;
mod index;

fn main() {
    let mut pages: Vec<(String, String)> = Vec::new();
    for file in source_file::SourceFile::from_dir(Path::new(".")).unwrap() {
        let body = file.body();
        let frontmatter = file.frontmatter();
        let output = layout::render(&body, frontmatter);

        let output_dir = Path::new("./_site");
        create_dir_all(output_dir).unwrap();

        pages.push(
            (file.output_path(Path::new("."), "html").display().to_string(), file.frontmatter().title)
        );

        let mut output_file = File::create(file.output_path(output_dir, "html")).unwrap();
        output_file.write_all(output.as_bytes()).unwrap();
    }

    let output_dir = Path::new("./_site");
    create_dir_all(output_dir).unwrap();
    let output = index::render(pages);

    let mut output_file = File::create(output_dir.clone().join("index.html")).unwrap();
    output_file.write_all(output.as_bytes()).unwrap();
}
