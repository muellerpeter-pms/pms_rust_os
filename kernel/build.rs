use std::process::Command;

fn main() {
    // nur erneut ausführen, wenn wir das build-script selbst verändern
    println!("cargo:rerun-if-changed=build-.rs");

    let _output = Command::new("rustup")
        .args(&["component", "add", "rust-src"])
        .output()
        .expect("rustup konnte nicht gestartet werden, um die rust-src zu laden");
}
