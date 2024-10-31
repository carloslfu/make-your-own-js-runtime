#![allow(clippy::print_stdout)]
#![allow(clippy::print_stderr)]

mod module_loader;

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use deno_runtime::deno_core::error::AnyError;
use deno_runtime::deno_core::op2;
use deno_runtime::deno_core::ModuleSpecifier;
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::worker::WorkerServiceOptions;
use module_loader::TypescriptModuleLoader;

#[op2(fast)]
fn op_hello(#[string] text: &str) {
    println!("Hello {} from an op!", text);
}

deno_runtime::deno_core::extension!(
  hello_runtime,
  ops = [op_hello],
  esm_entry_point = "ext:hello_runtime/bootstrap.js",
  esm = [dir "src", "bootstrap.js"]
);

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), AnyError> {
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("./fs.ts");
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();
    eprintln!("Running {main_module}...");
    let fs = Arc::new(RealFs);
    let permission_desc_parser = Arc::new(RuntimePermissionDescriptorParser::new(fs.clone()));

    let source_map_store = Rc::new(RefCell::new(HashMap::new()));

    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        WorkerServiceOptions {
            module_loader: Rc::new(TypescriptModuleLoader {
                source_maps: source_map_store,
            }),
            // File only loader
            // module_loader: Rc::new(FsModuleLoader),
            permissions: PermissionsContainer::allow_all(permission_desc_parser),
            blob_store: Default::default(),
            broadcast_channel: Default::default(),
            feature_checker: Default::default(),
            node_services: Default::default(),
            npm_process_state_provider: Default::default(),
            root_cert_store_provider: Default::default(),
            shared_array_buffer_store: Default::default(),
            compiled_wasm_module_store: Default::default(),
            v8_code_cache: Default::default(),
            fs,
        },
        WorkerOptions {
            extensions: vec![hello_runtime::init_ops_and_esm()],
            ..Default::default()
        },
    );
    println!("Bootstrapped worker");
    worker.execute_main_module(&main_module).await?;
    println!("Executed main module");
    worker.run_event_loop(false).await?;

    println!("Exit code: {}", worker.exit_code());

    Ok(())
}
