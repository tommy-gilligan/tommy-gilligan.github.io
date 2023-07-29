#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use std::{
    fs::{self, File},
    io::{prelude::*, BufReader},
    path::PathBuf,
};

mod layout;
mod prettier;
mod syntax_highlighting;

fn paths() -> Vec<(PathBuf, PathBuf)> {
    fs::read_dir(".")
        .unwrap()
        .map(std::result::Result::unwrap)
        .filter(|e| {
            e.file_type().unwrap().is_file()
                && e.path().extension().is_some()
                && e.path().extension().unwrap().to_str().unwrap() == "md"
        })
        .map(|f| {
            let input_path = f.path();
            let mut output_path = f.path();
            output_path.set_extension("html");
            (input_path, output_path)
        })
        .collect()
}

fn main() {
    for (input_path, output_path) in paths() {
        let input_file = File::open(input_path).unwrap();
        let mut output_file = File::create(output_path.clone()).unwrap();

        let mut buf_reader = BufReader::new(input_file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        output_file
            .write_all(layout::render(&contents).as_bytes())
            .unwrap();
        output_file.sync_all().unwrap();
    }

    let a = r#"
    for i in 1 2 3
    do
        echo $i
    done
    "#;
    println!("{}", syntax_highlighting::format_code(a));

    // prettier::run();
}
