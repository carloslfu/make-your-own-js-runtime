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
use deno_runtime::deno_permissions::set_prompter;
use deno_runtime::deno_permissions::PermissionPrompter;
use deno_runtime::deno_permissions::Permissions;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::deno_permissions::PromptResponse;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::worker::WorkerServiceOptions;

use colored::*;

use module_loader::TypescriptModuleLoader;

#[op2]
#[string]
fn example_custom_op(#[string] text: &str) -> String {
    println!("Hello {} from an op!", text);
    text.to_string() + " from Rust!"
}

deno_runtime::deno_core::extension!(
  example_extension,
  ops = [example_custom_op],
  esm_entry_point = "ext:example_extension/bootstrap.js",
  esm = [dir "src", "bootstrap.js"]
);

struct CustomPrompter;

impl PermissionPrompter for CustomPrompter {
    fn prompt(
        &mut self,
        message: &str,
        name: &str,
        api_name: Option<&str>,
        is_unary: bool,
    ) -> PromptResponse {
        println!(
            "{}\n{} {}\n{} {}\n{} {:?}\n{} {}",
            "Script is trying to access APIs and needs permission:"
                .yellow()
                .bold(),
            "Message:".bright_blue(),
            message,
            "Name:".bright_blue(),
            name,
            "API:".bright_blue(),
            api_name,
            "Is unary:".bright_blue(),
            is_unary
        );
        println!("Allow? [y/n]");

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => PromptResponse::Allow,
                _ => PromptResponse::Deny,
            }
        } else {
            println!("Failed to read input, denying permission");
            PromptResponse::Deny
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), AnyError> {
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("./test-files/all_the_things.ts");
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();

    let source_map_store = Rc::new(RefCell::new(HashMap::new()));

    let fs = Arc::new(RealFs);
    let permission_desc_parser = Arc::new(RuntimePermissionDescriptorParser::new(fs.clone()));
    let permission_container =
        PermissionsContainer::new(permission_desc_parser, Permissions::none_with_prompt());

    set_prompter(Box::new(CustomPrompter));

    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        WorkerServiceOptions {
            module_loader: Rc::new(TypescriptModuleLoader {
                source_maps: source_map_store,
            }),
            // File-only loader
            // module_loader: Rc::new(FsModuleLoader),
            permissions: permission_container,
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
            extensions: vec![example_extension::init_ops_and_esm()],
            ..Default::default()
        },
    );
    worker.execute_main_module(&main_module).await?;
    worker.run_event_loop(false).await?;

    println!("Exit code: {}", worker.exit_code());

    Ok(())
}
