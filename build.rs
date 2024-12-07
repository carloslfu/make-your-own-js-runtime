use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/bootstrap.js");

    deno_runtime::deno_core::extension!(
        example_extension,
        esm_entry_point = "ext:example_extension/bootstrap.js",
        esm = [dir "src", "bootstrap.js"],
    );

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let snapshot_path = out_dir.join("RUNJS_SNAPSHOT.bin");

    create_cli_snapshot(
        snapshot_path,
        vec![
            // placeholder
            example_extension::init_ops_and_esm(),
        ],
    );
}

fn create_cli_snapshot(snapshot_path: PathBuf, custom_extensions: Vec<deno_core::Extension>) {
    use deno_runtime::ops::bootstrap::SnapshotOptions;

    let snapshot_options = SnapshotOptions {
        ts_version: "5.6.2".to_string(), // replace with runtime version
        v8_version: deno_core::v8::VERSION_STRING,
        target: std::env::var("TARGET").unwrap(),
    };

    deno_runtime::snapshot::create_runtime_snapshot(
        snapshot_path,
        snapshot_options,
        custom_extensions,
    );
}
