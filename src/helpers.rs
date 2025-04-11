#![allow(dead_code)]
use cef::WrapApp;
use cef::WrapClient;
use cef::WrapRequestHandler;
use cef::WrapResourceRequestHandler;
use cef::WrapResponseFilter;
use cef::WrapBrowserProcessHandler;
use cef::WrapWindowDelegate;
use cef::rc::Rc;
use cef::rc::RcImpl;
use cef::sys;

use crate::app::DemoApp;
use crate::client::DemoClient;
use crate::filter::DemoResponseFilter;
use crate::xhr::{DemoRequestHandler, DemoResourceRequestHandler};
use crate::process::DemoBrowserProcessHandler;
use crate::window::DemoWindowDelegate;

pub trait LimitString {
    fn limit(&self, limit: usize) -> String;
}

impl LimitString for String {
    fn limit(&self, limit: usize) -> String {
        if self.len() > limit {
            format!("{}...", &self[..limit])
        } else {
            self.clone()
        }
    }
}

#[allow(dead_code)]
pub fn fmt_cef_string_utf16_t(s: &cef::sys::_cef_string_utf16_t) -> String {
    let slice = unsafe { std::slice::from_raw_parts(s.str_, s.length) };
    String::from_utf16_lossy(slice)
}

#[allow(dead_code)]
pub fn fmt_cef_string_utf16_userfree(s: &cef::CefStringUserfreeUtf16) -> String {
    let st = cef::CefString::from(s);
    st.to_string()
}

//
// DemoRequestHandler
//

/// Implementation of `WrapRequestHandler` for `DemoRequestHandler`
///
/// This trait implementation is required for CEF integration and allows
/// the Rust object to be wrapped in a CEF-compatible structure.
impl WrapRequestHandler for DemoRequestHandler {
    /// Wraps the handler in a CEF-compatible reference-counted object
    ///
    /// # Parameters
    /// - `object`: The raw pointer to the CEF request handler implementation
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_request_handler_t, Self>) {
        self.base = object;
    }
}

/// Clone implementation for `DemoRequestHandler`
///
/// Clones the handler by incrementing the CEF reference count.
impl Clone for DemoRequestHandler {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.base;
            rc_impl.interface.add_ref();
        }
        Self{
            base: self.base,
            config: self.config.clone(),
        }
    }
}

/// Reference counting implementation for `DemoRequestHandler`
///
/// Provides access to the base reference-counted object for CEF integration.
impl Rc for DemoRequestHandler {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.base;
            std::mem::transmute(&base.cef_object)
        }
    }
}

//
// DemoResourceRequestHandler
//

/// Implementation of `WrapResourceRequestHandler` for `DemoResourceRequestHandler`
///
/// This trait implementation is required for CEF integration and allows
/// the Rust object to be wrapped in a CEF-compatible structure.
impl WrapResourceRequestHandler for DemoResourceRequestHandler {
    /// Wraps the handler in a CEF-compatible reference-counted object
    ///
    /// # Parameters
    /// - `object`: The raw pointer to the CEF resource request handler implementation
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_resource_request_handler_t, Self>) {
        self.base = object;
    }
}

/// Clone implementation for `DemoResourceRequestHandler`
///
/// Clones the handler by incrementing the CEF reference count.
impl Clone for DemoResourceRequestHandler {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.base;
            rc_impl.interface.add_ref();
        }
        Self {
            base: self.base,
            config: self.config.clone(),
        }
    }
}

/// Reference counting implementation for `DemoResourceRequestHandler`
///
/// Provides access to the base reference-counted object for CEF integration.
impl Rc for DemoResourceRequestHandler {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.base;
            std::mem::transmute(&base.cef_object)
        }
    }
}

//
// DemoResponseFilter
//

/// Implementation of `WrapResponseFilter` for `DemoResponseFilter`
///
/// This trait implementation is required for CEF integration and allows
/// the Rust object to be wrapped in a CEF-compatible structure.
impl WrapResponseFilter for DemoResponseFilter {
    /// Wraps the filter in a CEF-compatible reference-counted object
    ///
    /// # Parameters
    /// - `object`: The raw pointer to the CEF response filter implementation
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_response_filter_t, Self>) {
        self.object = object;
    }
}

/// Clone implementation for `DemoResponseFilter`
///
/// Clones the filter by incrementing the CEF reference count and
/// creating a new filter with shared internal buffer.
impl Clone for DemoResponseFilter {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
        }
        Self {
            object: self.object,
            buffer: self.buffer.clone(),
            request_headers: self.request_headers.clone(),
            url: self.url.clone(),
            uuid: self.uuid,
            config: self.config.clone(),
        }
    }
}

/// Reference counting implementation for `DemoResponseFilter`
///
/// Provides access to the base reference-counted object for CEF integration.
impl Rc for DemoResponseFilter {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
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

        Self { object, window, config: self.config.clone() }
    }
}

//
// DemoWindowDelegate
//

impl WrapWindowDelegate for DemoWindowDelegate {
    /// Sets the raw CEF object pointer for this instance.
    ///
    /// This method is called by the CEF framework when wrapping the implementation
    /// in a reference-counted object.
    ///
    /// # Arguments
    /// * `object` - The raw CEF object pointer to set
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_window_delegate_t, Self>) {
        self.base = object;
    }
}

impl Clone for DemoWindowDelegate {
    /// Creates a clone of this window delegate instance.
    ///
    /// This implementation ensures proper reference counting of the underlying CEF object.
    ///
    /// # Returns
    /// A new `DemoWindowDelegate` instance that shares the same underlying CEF object
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.base;
            rc_impl.interface.add_ref();
        }

        Self {
            base: self.base,
            browser_view: self.browser_view.clone(),
            config: self.config.clone(),
        }
    }
}

impl Rc for DemoWindowDelegate {
    /// Accesses the base reference-counted object.
    ///
    /// This method is required by the CEF framework for reference counting.
    ///
    /// # Returns
    /// A reference to the base reference-counted object
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.base;
            std::mem::transmute(&base.cef_object)
        }
    }
}

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

        Self { object, window, config: self.config.clone() }
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

//
// DemoClient
//


impl WrapClient for DemoClient {
    /// Wraps the raw CEF client object.
    ///
    /// This method is called by the CEF framework during client initialization
    /// to associate the implementation with the underlying CEF object.
    ///
    /// # Parameters
    ///
    /// - `object`: The raw CEF client object pointer that this implementation will control.
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_client_t, Self>) {
        self.base = object;
    }
}

impl Clone for DemoClient {
    /// Clones the `DemoClient` instance.
    ///
    /// This implementation ensures proper reference counting of the underlying
    /// CEF object by incrementing its reference count when cloned.
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.base;
            rc_impl.interface.add_ref();
        }

        Self {
            base: self.base,
            config: self.config.clone(),
        }
    }
}

impl Rc for DemoClient {
    /// Returns the base reference-counted CEF object.
    ///
    /// This method provides access to the CEF reference counting interface,
    /// which is required for proper memory management within the CEF framework.
    ///
    /// # Returns
    ///
    /// A reference to the base reference-counted structure of the CEF object.
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.base;
            std::mem::transmute(&base.cef_object)
        }
    }
}