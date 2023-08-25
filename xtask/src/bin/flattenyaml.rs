use std::io::Read;

fn flatten_yaml<W>(input: clio::Input, output: &mut W) -> serde_yaml::Result<()>
where
    W: std::io::Write,
{
    let flattened: serde_yaml::Value = serde_yaml::from_reader(input)?;
    serde_yaml::to_writer(output, &flattened)
}

fn main() -> clio::Result<()> {
    let mut args = std::env::args_os().skip(1);
    let check_flag = std::ffi::OsString::from("--check");

    match args.next() {
        Some(flag) if flag == check_flag => {
            let input = clio::Input::new(&args.next().expect("Input argument absent"))?;
            let mut output = clio::Input::new(&args.next().expect("Output argument absent"))?;

            let mut new_buffer: Vec<u8> = Vec::new();
            flatten_yaml(input, &mut new_buffer).unwrap();

            let mut old_buffer: Vec<u8> = Vec::new();
            output.read_to_end(&mut old_buffer).unwrap();

            assert!(old_buffer == new_buffer);
        }
        Some(input_arg) => {
            let input = clio::Input::new(&input_arg)?;
            let mut output = clio::Output::new(&args.next().expect("Output argument absent"))?;

            flatten_yaml(input, &mut output).unwrap();
        }
        _ => panic!("panic"),
    }

    Ok(())
}
