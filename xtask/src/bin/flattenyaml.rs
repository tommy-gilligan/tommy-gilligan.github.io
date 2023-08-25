use std::path::Path;
mod flatten_yaml;

fn main() {
    let mut args = std::env::args_os().skip(1);
    let check_flag = std::ffi::OsString::from("--check");

    match args.next() {
        Some(flag) if flag == check_flag => {
            let input = args.next().expect("Input argument absent");
            let output = args.next().expect("Output argument absent");

            crate::flatten_yaml::check(Path::new(&input), Path::new(&output));
        }
        Some(input_arg) => {
            let output = args.next().expect("Output argument absent");

            crate::flatten_yaml::flatten_yaml(
                &mut std::fs::File::open(Path::new(&input_arg)).unwrap(),
                &mut std::fs::File::create(Path::new(&output)).unwrap(),
            )
            .unwrap();
        }
        _ => panic!("panic"),
    }
}
