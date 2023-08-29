use std::{
    env::args_os,
    ffi::OsString,
    fs::{read, File},
    io::{Read, Write},
    path::Path,
};

pub fn flatten_yaml<R, W>(input: &mut R, output: &mut W) -> serde_yaml::Result<()>
where
    R: Read,
    W: Write,
{
    let flattened: serde_yaml::Value = serde_yaml::from_reader(input)?;
    serde_yaml::to_writer(output, &flattened)
}

pub fn check(source: &Path, target: &Path) -> bool {
    let mut new_buffer: Vec<u8> = Vec::new();

    flatten_yaml(&mut File::open(source).unwrap(), &mut new_buffer).unwrap();
    if read(target).unwrap() != new_buffer {
        eprintln!(
            "{} needs to be flattened to {}",
            source.display(),
            target.display()
        );
        return false;
    }
    true
}

#[allow(dead_code)]
fn main() {
    let mut args = args_os().skip(1);
    let check_flag = OsString::from("--check");

    match args.next() {
        Some(flag) if flag == check_flag => {
            let input = args.next().expect("Input argument absent");
            let output = args.next().expect("Output argument absent");

            check(Path::new(&input), Path::new(&output));
        }
        Some(input_arg) => {
            let output = args.next().expect("Output argument absent");

            flatten_yaml(
                &mut File::open(Path::new(&input_arg)).unwrap(),
                &mut File::create(Path::new(&output)).unwrap(),
            )
            .unwrap();
        }
        _ => panic!("panic"),
    }
}
