//! Chromium Embedded Framework (CEF) application implementation.
//!
//! This module defines the main application entry point for the CEF framework.
//! It establishes the primary integration point with the CEF browser process.

use std::sync::{Arc, Mutex};

use cef::{
    App, BrowserProcessHandler, ImplApp, Window, WrapApp,
    rc::{Rc, RcImpl},
    sys,
};

use crate::process::DemoBrowserProcessHandler;

/// Main CEF application implementation.
///
/// `DemoApp` serves as the primary application class that integrates with the CEF framework.
/// It manages the application lifecycle and provides access to process-specific handlers.
///
/// # Fields
/// * `object` - The raw CEF object pointer for reference counting
/// * `window` - A thread-safe reference to the application's main window
pub struct DemoApp {
    object: *mut RcImpl<sys::_cef_app_t, Self>,
    window: Arc<Mutex<Option<Window>>>,
}

impl DemoApp {
    /// Creates a new CEF application instance.
    ///
    /// # Arguments
    /// * `window` - A thread-safe reference to the application's main window (initially None)
    ///
    /// # Returns
    /// A new `App` instance wrapping the `DemoApp` implementation
    pub fn new(window: Arc<Mutex<Option<Window>>>) -> App {
        App::new(Self {
            object: std::ptr::null_mut(),
            window,
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
        Some(DemoBrowserProcessHandler::new(self.window.clone()))
    }
}

//
//
//
//
// //////////////////////////////
// /          HELPERS           /
// //////////////////////////////
//
//
//
//

//
// DemoApp
//

impl WrapApp for DemoApp {
    /// Sets the raw CEF object pointer for this instance.
    ///
    /// This method is called by the CEF framework when wrapping the implementation
    /// in a reference-counted object.
    ///
    /// # Arguments
    /// * `object` - The raw CEF object pointer to set
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_app_t, Self>) {
        self.object = object;
    }
}

impl Clone for DemoApp {
    /// Creates a clone of this application instance.
    ///
    /// This implementation ensures proper reference counting of the underlying CEF object.
    ///
    /// # Returns
    /// A new `DemoApp` instance that shares the same underlying CEF object
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            self.object
        };
        let window = self.window.clone();

        Self { object, window }
    }
}

impl Rc for DemoApp {
    /// Accesses the base reference-counted object.
    ///
    /// This method is required by the CEF framework for reference counting.
    ///
    /// # Returns
    /// A reference to the base reference-counted object
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}
