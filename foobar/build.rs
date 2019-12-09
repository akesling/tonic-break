fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "../proto";
    println!("cargo:rerun-if-changed={}", proto_root);

    let proto_files = &["foo.proto", "bar.proto"];

    tonic_build::configure()
        .build_server(false)
        .compile(proto_files, &[proto_root])?;
    Ok(())
}
