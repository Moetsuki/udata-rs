#![allow(clippy::new_ret_no_self)]
//! Client implementation for Chromium Embedded Framework (CEF) integration.
//! 
//! This module provides a custom client implementation for CEF that handles
//! browser-related events and requests. The `DemoClient` serves as the primary
//! interface between the application and the embedded browser instances.

use cef::{Client, ImplClient, rc::RcImpl, sys, RequestHandler};
use crate::{config::Config, xhr::DemoRequestHandler};

/// A custom implementation of `Client` for handling browser interactions.
///
/// This client implementation manages the connection between the application and
/// CEF browser instances, providing handlers for various browser events and requests.
/// It implements the necessary traits to work within CEF's reference-counting system.
pub struct DemoClient {
    pub base: *mut RcImpl<sys::_cef_client_t, Self>,
    pub config: Option<Config>,
}

impl DemoClient {
    /// Creates a new instance of `DemoClient`.
    ///
    /// # Returns
    ///
    /// Returns a new `Client` instance wrapping our custom implementation.
    /// This client can be used when creating browser instances.
    ///
    pub fn new(config: Option<Config>) -> Client {
        Client::new(Self {
            base: std::ptr::null_mut(),
            config
        })
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
        Some(DemoRequestHandler::new(self.config.clone()))
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
        self.base as *mut sys::_cef_client_t
    }
}
