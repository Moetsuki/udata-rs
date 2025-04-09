use cef::{Client, ImplClient, WrapClient, rc::{Rc, RcImpl}, sys, RequestHandler};
use crate::xhr::DemoRequestHandler;

/// A custom implementation of `Client` for handling browser interactions.
pub struct DemoClient(*mut RcImpl<sys::_cef_client_t, Self>);

impl DemoClient {
    /// Creates a new instance of `DemoClient`.
    pub fn new() -> Client {
        Client::new(Self(std::ptr::null_mut()))
    }
}

impl WrapClient for DemoClient {
    /// Wraps the raw CEF client object.
    ///
    /// # Parameters
    /// - `object`: The raw CEF client object.
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_client_t, Self>) {
        self.0 = object;
    }
}

impl Clone for DemoClient {
    /// Clones the `DemoClient` instance.
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
    /// # Returns
    /// An optional `RequestHandler` instance.
    fn get_request_handler(&self) -> Option<RequestHandler> {
        Some(DemoRequestHandler::new())
    }

    /// Returns the raw pointer to the underlying CEF client.
    fn get_raw(&self) -> *mut sys::_cef_client_t {
        self.0 as *mut sys::_cef_client_t
    }
}
