#![allow(clippy::new_ret_no_self)]
//! Browser process handling for CEF integration.
//!
//! This module contains the implementation of the browser process handler,
//! which is responsible for handling events in the main browser process.
//! It manages browser initialization, context setup, and window creation.

use std::sync::{Arc, Mutex};
use cef::rc::RcImpl;
use cef::{
    BrowserProcessHandler, BrowserViewDelegate, CefString, DictionaryValue,
    ImplBrowserProcessHandler, RequestContext, Window, browser_view_create,
    sys, window_create_top_level,
};

use crate::config::Config;
use crate::{client::DemoClient, window::DemoWindowDelegate};

/// Handler for browser process events.
///
/// `DemoBrowserProcessHandler` is responsible for managing browser process lifecycle
/// events, particularly the initialization of the browser context and creation of
/// the initial browser window.
///
/// # Fields
/// * `object` - The raw CEF object pointer for reference counting
/// * `window` - A thread-safe reference to the application's main window
pub struct DemoBrowserProcessHandler {
    pub object: *mut RcImpl<sys::cef_browser_process_handler_t, Self>,
    pub window: Arc<Mutex<Option<Window>>>,
}

impl DemoBrowserProcessHandler {
    /// Creates a new browser process handler instance.
    ///
    /// # Arguments
    /// * `window` - A thread-safe reference to the application's main window (initially None)
    ///
    /// # Returns
    /// A new `BrowserProcessHandler` instance wrapping the `DemoBrowserProcessHandler` implementation
    pub fn new(window: Arc<Mutex<Option<Window>>>) -> BrowserProcessHandler {
        BrowserProcessHandler::new(Self {
            object: std::ptr::null_mut(),
            window,
        })
    }
}

impl ImplBrowserProcessHandler for DemoBrowserProcessHandler {
    /// Returns the raw CEF browser process handler pointer.
    ///
    /// # Returns
    /// A pointer to the underlying CEF browser process handler structure
    fn get_raw(&self) -> *mut sys::_cef_browser_process_handler_t {
        self.object.cast()
    }

    /// Called when the browser context has been initialized.
    ///
    /// The real lifespan of CEF starts from this method, so all CEF objects should
    /// be created and manipulated after this point. This method initializes the
    /// browser view, creates a client, and sets up the main application window.
    fn on_context_initialized(&self) {
        println!("cef context intiialized");

        let config: Option<Config> = serde_json::from_reader(
            std::fs::File::open(".udata/config.json").expect("Failed to open config.json"),
        ).ok();
        
        let mut client = DemoClient::new(config.clone());
        
        let url = CefString::from("https://www.yahoo.com");

        // Create a browser view with the client and URL
        let browser_view = browser_view_create(
            Some(&mut client),
            Some(&url),
            Some(&Default::default()),
            Option::<&mut DictionaryValue>::None,
            Option::<&mut RequestContext>::None,
            Option::<&mut BrowserViewDelegate>::None,
        )
        .expect("Failed to create browser view");

        // Create a window delegate and window
        let mut delegate = DemoWindowDelegate::new(browser_view, config);
        if let Ok(mut window) = self.window.lock() {
            *window = Some(
                window_create_top_level(Some(&mut delegate)).expect("Failed to create window"),
            );
        }
    }
}
