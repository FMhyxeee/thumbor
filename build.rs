fn main() {

    // println!("cargo:rerun-if-changed=abi.proto");
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
