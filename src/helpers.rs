use cef::WrapRequestHandler;
use cef::WrapResourceRequestHandler;
use cef::WrapResponseFilter;
use cef::rc::Rc;
use cef::rc::RcImpl;
use cef::sys;

use crate::filter::DemoResponseFilter;
use crate::xhr::{DemoRequestHandler, DemoResourceRequestHandler};

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
        self.0 = object;
    }
}

/// Clone implementation for `DemoRequestHandler`
///
/// Clones the handler by incrementing the CEF reference count.
impl Clone for DemoRequestHandler {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }
        Self(self.0)
    }
}

/// Reference counting implementation for `DemoRequestHandler`
///
/// Provides access to the base reference-counted object for CEF integration.
impl Rc for DemoRequestHandler {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
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
        self.0 = object;
    }
}

/// Clone implementation for `DemoResourceRequestHandler`
///
/// Clones the handler by incrementing the CEF reference count.
impl Clone for DemoResourceRequestHandler {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }
        Self(self.0)
    }
}

/// Reference counting implementation for `DemoResourceRequestHandler`
///
/// Provides access to the base reference-counted object for CEF integration.
impl Rc for DemoResourceRequestHandler {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
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
