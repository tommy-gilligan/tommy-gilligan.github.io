fn flatten_yaml(input: clio::Input, output: clio::Output) -> serde_yaml::Result<()> {
    let flattened: serde_yaml::Value = serde_yaml::from_reader(input)?;
    serde_yaml::to_writer(output, &flattened)
}

fn main() -> clio::Result<()> {
    let mut args = std::env::args_os().skip(1);
    let input = clio::Input::new(&args.next().expect("Input argument absent"))?;
    let output = clio::Output::new(&args.next().expect("Output argument absent"))?;
    flatten_yaml(input, output).unwrap();

    Ok(())
}
