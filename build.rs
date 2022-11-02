extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=openfirmware/clients/lib/1275.h");

    let bindings = bindgen::Builder::default()
        .use_core()
        .header("openfirmware/clients/lib/1275.h")
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from("src/".to_string());
    bindings.write_to_file(out_path.join("l1275.rs"))
        .expect("couldn't write bindings!");
}