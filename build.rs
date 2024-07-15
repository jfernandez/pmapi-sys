use std::{env, path::PathBuf};

fn main() {
    // Add library linking directives
    println!("cargo:rustc-link-lib=pcp");
    
    let bindings = bindgen::Builder::default()
        .header("bindings.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // IPPORT_RESERVED causes a redefined error
        .blocklist_item("IPPORT_RESERVED")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
