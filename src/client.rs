#![allow(clippy::new_ret_no_self)]
//! Client implementation for Chromium Embedded Framework (CEF) integration.
//! 
//! This module provides a custom client implementation for CEF that handles
//! browser-related events and requests. The `DemoClient` serves as the primary
//! interface between the application and the embedded browser instances.

use cef::{Client, ImplClient, WrapClient, rc::{Rc, RcImpl}, sys, RequestHandler};
use crate::xhr::DemoRequestHandler;

/// A custom implementation of `Client` for handling browser interactions.
///
/// This client implementation manages the connection between the application and
/// CEF browser instances, providing handlers for various browser events and requests.
/// It implements the necessary traits to work within CEF's reference-counting system.
pub struct DemoClient(*mut RcImpl<sys::_cef_client_t, Self>);

impl DemoClient {
    /// Creates a new instance of `DemoClient`.
    ///
    /// # Returns
    ///
    /// Returns a new `Client` instance wrapping our custom implementation.
    /// This client can be used when creating browser instances.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let client = DemoClient::new();
    /// // Use client when creating a browser window
    /// ```
    pub fn new() -> Client {
        Client::new(Self(std::ptr::null_mut()))
    }
}

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
        self.0 = object;
    }
}

impl Clone for DemoClient {
    /// Clones the `DemoClient` instance.
    ///
    /// This implementation ensures proper reference counting of the underlying
    /// CEF object by incrementing its reference count when cloned.
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }

        Self(self.0)
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
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplClient for DemoClient {
    /// Provides the request handler for the client.
    ///
    /// This method is called by the CEF framework when it needs to handle
    /// network requests. Our implementation returns a `DemoRequestHandler`
    /// that can intercept and process HTTP requests and responses.
    ///
    /// # Returns
    ///
    /// An optional `RequestHandler` instance. Returns `Some` with our custom
    /// request handler implementation.
    fn get_request_handler(&self) -> Option<RequestHandler> {
        Some(DemoRequestHandler::new())
    }

    /// Returns the raw pointer to the underlying CEF client.
    ///
    /// This method provides access to the raw CEF client structure,
    /// which may be needed when interacting with low-level CEF APIs.
    ///
    /// # Returns
    ///
    /// A mutable raw pointer to the underlying CEF client structure.
    fn get_raw(&self) -> *mut sys::_cef_client_t {
        self.0 as *mut sys::_cef_client_t
    }
}
