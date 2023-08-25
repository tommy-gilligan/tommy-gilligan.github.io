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
