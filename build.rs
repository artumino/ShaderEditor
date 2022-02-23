use std::process::Command;

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=ShaderEditor.cs");
    Command::new("dotnet").args(&["build", "--configuration", "Release"])
                       .status().unwrap();
}