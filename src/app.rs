#![allow(clippy::new_ret_no_self)]
//! Chromium Embedded Framework (CEF) application implementation.
//!
//! This module defines the main application entry point for the CEF framework.
//! It establishes the primary integration point with the CEF browser process.

use std::sync::{Arc, Mutex};

use cef::{
    App, BrowserProcessHandler, ImplApp, Window,
    rc::RcImpl,
    sys,
};

use crate::{config::Config, process::DemoBrowserProcessHandler};

/// Main CEF application implementation.
///
/// `DemoApp` serves as the primary application class that integrates with the CEF framework.
/// It manages the application lifecycle and provides access to process-specific handlers.
///
/// # Fields
/// * `object` - The raw CEF object pointer for reference counting
/// * `window` - A thread-safe reference to the application's main window
pub struct DemoApp {
    pub object: *mut RcImpl<sys::_cef_app_t, Self>,
    pub window: Arc<Mutex<Option<Window>>>,
    pub config: Option<Config>,
}

impl DemoApp {
    /// Creates a new CEF application instance.
    ///
    /// # Arguments
    /// * `window` - A thread-safe reference to the application's main window (initially None)
    ///
    /// # Returns
    /// A new `App` instance wrapping the `DemoApp` implementation
    pub fn new(window: Arc<Mutex<Option<Window>>>, config: Option<Config>) -> App {
        App::new(Self {
            object: std::ptr::null_mut(),
            window,
            config,
        })
    }
}

impl ImplApp for DemoApp {
    /// Returns the raw CEF application pointer.
    ///
    /// # Returns
    /// A pointer to the underlying CEF application structure
    fn get_raw(&self) -> *mut sys::_cef_app_t {
        self.object as *mut sys::_cef_app_t
    }

    /// Provides the browser process handler for this application.
    ///
    /// This method is called by the CEF framework to obtain the browser process handler
    /// that will handle browser process-specific callbacks.
    ///
    /// # Returns
    /// An instance of `DemoBrowserProcessHandler` wrapped in `BrowserProcessHandler`
    fn get_browser_process_handler(&self) -> Option<BrowserProcessHandler> {
        Some(DemoBrowserProcessHandler::new(self.window.clone(), self.config.clone()))
    }
}
