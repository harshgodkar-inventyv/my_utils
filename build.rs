use std::{path::Path, process::Command};

fn main() {
    let proto_repo_url = "https://github.com/harshgodkar-inventyv/my_proto_schema.git";
    let proto_dir = "proto";

    clone_proto_repo(proto_repo_url, proto_dir);
    // Compile proto files - note the correct path
    prost_build::Config::new()
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir(std::env::var("OUT_DIR").unwrap())
        .compile_protos(
            &["./proto/myaap/user.proto", "./proto/common/common.proto"],// Updated path
            &[proto_dir],            // Include path
        )
        .unwrap();
}


fn clone_proto_repo(repo_url: &str, target_dir: &str) {
    let path = Path::new(target_dir);
 
    // If the proto directory exists, remove it
    if path.exists() {
        println!("cargo:warning=Removing existing proto directory...");
        std::fs::remove_dir_all(path).expect("Failed to remove existing proto directory");
    }
 
    // Clone the repository
    println!("cargo:warning=Cloning proto repository from {}...", repo_url);
    let status = Command::new("git")
        .args(&["clone", repo_url, target_dir])
        .status()
        .expect("Failed to execute git clone command");
 
    if !status.success() {
        panic!("Failed to clone proto repository from {}", repo_url);
    }
 
    println!("cargo:warning=Proto repository cloned successfully");
 
    // Tell Cargo to rerun this build script if the proto directory changes
    // println!("cargo:rerun-if-changed={}", target_dir);
}






// use std::env;
// use std::path::PathBuf;

// fn main() {
//     let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

//     let proto_path = PathBuf::from(&manifest_dir)
//         .join("..") // adjust if needed
//         .join("proto_schema") // crate name folder

//     prost_build::compile_protos(
//         &[
//             proto_path.join("crm/user.proto"),
//             proto_path.join("login/hello.proto"),
//         ],
//         &[proto_path],
//     ).unwrap();
// }
