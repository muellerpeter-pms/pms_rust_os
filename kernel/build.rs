use std::process::Command;

fn main() {
    // nur erneut ausführen, wenn wir das build-script selbst verändern
    println!("cargo:rerun-if-changed=build-.rs");

    let _output = Command::new("rustup")
        .args(&["component", "add", "rust-src"])
        .output()
        .expect("rustup konnte nicht gestartet werden, um die rust-src zu laden");

    Command::new("rustup")
        .args(&["component", "add", "llvm-tools-preview"])
        .output()
        .expect("rustup konnte nicht gestartet werden, um die llvm-tools-preview zu laden");

    Command::new("cargo")
        .args(&["install", "bootimage"])
        .output()
        .expect("cargo konnte nicht verwendet werden, um den bootloader zu installieren");
}
