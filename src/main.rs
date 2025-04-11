mod app;
mod client;
mod filter;
mod helpers;
mod process;
mod tests;
mod window;
mod xhr;
mod swizzle;
mod config;

use std::sync::{Arc, Mutex};

use colored::Colorize;

use app::DemoApp;
use cef::args::Args;
use cef::rc::Rc;
use cef::sandbox_info::SandboxInfo;
use cef::{Settings, api_hash, execute_process, initialize, run_message_loop, shutdown, sys};
use config::Config;

///
/// In order for this example to work you must manually go to
///
/// target/debug/build/cef ***/out/cef_linux_x86_64/
///
/// and set the executable `chrome-sandbox`
///
/// sudo chown root:root chrome-sandbox
/// sudo chmod 4755 chrome-sandbox
///
fn main() {
    println!("Starting udata-rs program");

    let _ = api_hash(sys::CEF_API_VERSION_LAST, 0);

    let _args = Args::new();

    let _sandbox = SandboxInfo::new();

    // print cwd
    let cwd = std::env::current_dir().unwrap();
    println!("[{}] Current working directory: {:?}", "success".green() ,cwd);
    
    let config: Option<Config> = serde_json::from_reader({
        std::fs::File::open(".udata/settings.json").unwrap_or_else(|_| {
            println!("[{}] Failed to open settings.json, attempting once more.", "warning".yellow());
    
            // change working dir
            std::env::set_current_dir("../").expect("Failed to change working directory");
            let cwd = std::env::current_dir().unwrap();
            println!("[{}] Current working directory: {:?}", "success".green(), cwd);
            
            std::fs::File::create(".udata/settings.json").expect("Failed to create settings.json") 
        })
    }).ok();

    if let Some(config) = config.as_ref() {
        println!("[{}] {:?}", "success".green(), config);
    } else {
        println!("[{}] No config found", "error".red());
    }

    let window = Arc::new(Mutex::new(None));
    let mut app = DemoApp::new(window.clone(), config);

    let _ret = execute_process(
        Some(_args.as_main_args()),
        Some(&mut app),
        _sandbox.as_mut_ptr(),
    );

    let settings = Settings::default();
    assert_eq!(
        initialize(
            Some(_args.as_main_args()),
            Some(&settings),
            Some(&mut app),
            _sandbox.as_mut_ptr()
        ),
        1
    );

    run_message_loop();

    let window = window.lock().expect("Failed to lock window");
    let window = window.as_ref().expect("Window is None");
    assert!(window.has_one_ref());

    shutdown();
}
