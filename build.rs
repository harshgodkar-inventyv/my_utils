fn main() {
 
    // Compile proto files - note the correct path
    prost_build::Config::new()
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir(std::env::var("OUT_DIR").unwrap())
        .compile_protos(
            &["./proto/user.proto"],  // Updated path
            &["./proto"]  // Include path
        )
        .unwrap();
}