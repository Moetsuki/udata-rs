#![allow(clippy::new_ret_no_self)]
//! Window management for CEF integration.
//!
//! This module contains the implementation of window-related delegates that
//! handle the window lifecycle, layout, and integration with the browser view.
//! It manages window creation, destruction, and interaction with the UI.

use crate::config::Config;

use cef::{
    BrowserView, ImplPanelDelegate, ImplView, ImplViewDelegate, ImplWindow, ImplWindowDelegate,
    WindowDelegate, quit_message_loop,
    rc::RcImpl,
    sys,
};

/// Window delegate implementation for the main application window.
///
/// `DemoWindowDelegate` handles window lifecycle events and integrates the
/// browser view with the window. It manages window creation, layout, and destruction.
///
/// # Fields
/// * `base` - The raw CEF object pointer for reference counting
/// * `browser_view` - The browser view to be displayed in this window
pub struct DemoWindowDelegate {
    pub base: *mut RcImpl<sys::_cef_window_delegate_t, Self>,
    pub browser_view: BrowserView,
    pub config: Option<Config>,
}

impl DemoWindowDelegate {
    /// Creates a new window delegate instance.
    ///
    /// # Arguments
    /// * `browser_view` - The browser view to be displayed in the window
    ///
    /// # Returns
    /// A new `WindowDelegate` instance wrapping the `DemoWindowDelegate` implementation
    pub fn new(browser_view: BrowserView, config: Option<Config>) -> WindowDelegate {
        WindowDelegate::new(Self {
            base: std::ptr::null_mut(),
            browser_view,
            config,
        })
    }
}

//
// View
//

impl ImplViewDelegate for DemoWindowDelegate {
    /// Called when a child view is added or removed.
    ///
    /// # Arguments
    /// * `_view` - The view that has changed
    /// * `_added` - Whether the child view was added (1) or removed (0)
    /// * `_child` - The child view that was added or removed
    fn on_child_view_changed(
        &self,
        _view: Option<&mut impl ImplView>,
        _added: ::std::os::raw::c_int,
        _child: Option<&mut impl ImplView>,
    ) {
        // view.as_panel().map(|x| x.as_window().map(|w| w.close()));
    }

    /// Returns the raw CEF view delegate pointer.
    ///
    /// # Returns
    /// A pointer to the underlying CEF view delegate structure
    fn get_raw(&self) -> *mut sys::_cef_view_delegate_t {
        self.base as *mut sys::_cef_view_delegate_t
    }
}

//
// Panel
//

/// Implementation of the panel delegate interface.
///
/// Currently, this is a blank implementation as we don't need custom panel behavior.
impl ImplPanelDelegate for DemoWindowDelegate {}

//
// Window
//

impl ImplWindowDelegate for DemoWindowDelegate {
    /// Called when the window has been created.
    ///
    /// This method is called after the native window has been created. It adds
    /// the browser view to the window and makes the window visible.
    ///
    /// # Arguments
    /// * `window` - The window that was created
    fn on_window_created(&self, window: Option<&mut impl ImplWindow>) {
        if let Some(window) = window {
            let mut view = self.browser_view.clone();
            window.add_child_view(Some(&mut view));
            window.show();
        }
    }

    /// Called when the window is being destroyed.
    ///
    /// This method is called when the window is about to be destroyed. It quits
    /// the message loop to terminate the application.
    ///
    /// # Arguments
    /// * `_window` - The window that is being destroyed
    fn on_window_destroyed(&self, _window: Option<&mut impl ImplWindow>) {
        quit_message_loop();
    }

    /// Indicates whether the window should have standard window buttons.
    ///
    /// # Arguments
    /// * `_window` - The window being configured
    ///
    /// # Returns
    /// 1 if the window should have standard buttons, 0 otherwise
    fn with_standard_window_buttons(
        &self,
        _window: Option<&mut impl ImplWindow>,
    ) -> ::std::os::raw::c_int {
        1
    }

    /// Indicates whether the window can be resized.
    ///
    /// # Arguments
    /// * `_window` - The window being configured
    ///
    /// # Returns
    /// 1 if the window can be resized, 0 otherwise
    fn can_resize(&self, _window: Option<&mut impl ImplWindow>) -> ::std::os::raw::c_int {
        1
    }

    /// Indicates whether the window can be maximized.
    ///
    /// # Arguments
    /// * `_window` - The window being configured
    ///
    /// # Returns
    /// 1 if the window can be maximized, 0 otherwise
    fn can_maximize(&self, _window: Option<&mut impl ImplWindow>) -> ::std::os::raw::c_int {
        1
    }

    /// Indicates whether the window can be minimized.
    ///
    /// # Arguments
    /// * `_window` - The window being configured
    ///
    /// # Returns
    /// 1 if the window can be minimized, 0 otherwise
    fn can_minimize(&self, _window: Option<&mut impl ImplWindow>) -> ::std::os::raw::c_int {
        1
    }

    /// Indicates whether the window can be closed.
    ///
    /// # Arguments
    /// * `_window` - The window being configured
    ///
    /// # Returns
    /// 1 if the window can be closed, 0 otherwise
    fn can_close(&self, _window: Option<&mut impl ImplWindow>) -> ::std::os::raw::c_int {
        1
    }
}