#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use std::{
    fs::{create_dir_all, File},
    io::prelude::*,
    path::Path,
};

mod layout;
mod source_file;

fn main() {
    for file in source_file::SourceFile::from_dir(Path::new(".")).unwrap() {
        let body = file.body();
        let frontmatter = file.frontmatter();
        let output = layout::render(&body, frontmatter);

        let output_dir = Path::new("./_site");
        create_dir_all(output_dir).unwrap();
        let mut output_file = File::create(file.output_path(output_dir, "html")).unwrap();
        output_file.write_all(output.as_bytes()).unwrap();
    }
}
