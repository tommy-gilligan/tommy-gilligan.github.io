use std::path::Path;

pub fn flatten_yaml<R, W>(input: &mut R, output: &mut W) -> serde_yaml::Result<()>
where
    R: std::io::Read,
    W: std::io::Write,
{
    let flattened: serde_yaml::Value = serde_yaml::from_reader(input)?;
    serde_yaml::to_writer(output, &flattened)
}

pub fn check(source: &std::path::Path, target: &std::path::Path) {
    let mut new_buffer: Vec<u8> = Vec::new();

    flatten_yaml(&mut std::fs::File::open(source).unwrap(), &mut new_buffer).unwrap();
    assert!(std::fs::read(target).unwrap() == new_buffer);
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let check_flag = std::ffi::OsString::from("--check");

    match args.next() {
        Some(flag) if flag == check_flag => {
            let input = args.next().expect("Input argument absent");
            let output = args.next().expect("Output argument absent");

            check(Path::new(&input), Path::new(&output));
        }
        Some(input_arg) => {
            let output = args.next().expect("Output argument absent");

            flatten_yaml(
                &mut std::fs::File::open(Path::new(&input_arg)).unwrap(),
                &mut std::fs::File::create(Path::new(&output)).unwrap(),
            )
            .unwrap();
        }
        _ => panic!("panic"),
    }
}
