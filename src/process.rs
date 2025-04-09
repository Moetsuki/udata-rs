//! Browser process handling for CEF integration.
//!
//! This module contains the implementation of the browser process handler,
//! which is responsible for handling events in the main browser process.
//! It manages browser initialization, context setup, and window creation.

use std::sync::{Arc, Mutex};

use cef::{
    BrowserProcessHandler, BrowserViewDelegate, CefString, DictionaryValue,
    ImplBrowserProcessHandler, RequestContext, Window, WrapBrowserProcessHandler,
    browser_view_create,
    rc::{Rc, RcImpl},
    sys, window_create_top_level,
};

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
    object: *mut RcImpl<sys::cef_browser_process_handler_t, Self>,
    window: Arc<Mutex<Option<Window>>>,
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

impl Rc for DemoBrowserProcessHandler {
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

impl WrapBrowserProcessHandler for DemoBrowserProcessHandler {
    /// Sets the raw CEF object pointer for this instance.
    ///
    /// This method is called by the CEF framework when wrapping the implementation
    /// in a reference-counted object.
    ///
    /// # Arguments
    /// * `object` - The raw CEF object pointer to set
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_browser_process_handler_t, Self>) {
        self.object = object;
    }
}

impl Clone for DemoBrowserProcessHandler {
    /// Creates a clone of this browser process handler instance.
    ///
    /// This implementation ensures proper reference counting of the underlying CEF object.
    ///
    /// # Returns
    /// A new `DemoBrowserProcessHandler` instance that shares the same underlying CEF object
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            rc_impl
        };

        let window = self.window.clone();

        Self { object, window }
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
        let mut client = DemoClient::new();
        let url = CefString::from("https://www.google.com");

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
        let mut delegate = DemoWindowDelegate::new(browser_view);
        if let Ok(mut window) = self.window.lock() {
            *window = Some(
                window_create_top_level(Some(&mut delegate)).expect("Failed to create window"),
            );
        }
    }
}
