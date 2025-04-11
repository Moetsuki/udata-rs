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

use app::DemoApp;
use cef::args::Args;
use cef::rc::Rc;
use cef::sandbox_info::SandboxInfo;
use cef::{Settings, api_hash, execute_process, initialize, run_message_loop, shutdown, sys};

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

    let window = Arc::new(Mutex::new(None));
    let mut app = DemoApp::new(window.clone());

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
