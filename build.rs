use std::{env, fs, path::Path};

fn main() {
    rust_witness::transpile::transpile_wasm("./test-vectors/circom".to_string());
    // witnesscalc_adapter::build_and_link("./test-vectors/circom");

    // Create the stub source file
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let stub_path = Path::new(&out_dir).join("chkstk_stub.c");
    fs::write(
        &stub_path,
        r#"
        void __chkstk_darwin(void) {}
        "#,
    )
    .unwrap();

    // Compile the stub into a static lib
    cc::Build::new().file(&stub_path).compile("chkstk_stub");
    println!("cargo:rustc-link-lib=static=chkstk_stub");
}
