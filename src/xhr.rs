#![allow(clippy::new_ret_no_self)]
//! # XHR Request Handling Module
//!
//! This module provides implementations for handling XMLHttpRequests (XHR) in a CEF environment.
//! It includes request handlers, resource request handlers, and response filters that intercept,
//! monitor, and potentially modify XHR requests and responses.
//!
//! The module implements a chain of responsibility pattern where:
//! 1. `DemoRequestHandler` is the entry point for all requests
//! 2. `DemoResourceRequestHandler` processes specific resource requests
//! 3. `DemoResponseFilter` in filter.rs filters and modifies response data
//!
//! ## Key Components:
//! - Request interception and header manipulation
//! - Response filtering and content processing
//! - Diagnostic logging for debugging XHR traffic
use cef::sys::cef_return_value_t;
use cef::{
    CefString, ImplBrowser, ImplFrame, ImplRequest, ImplRequestHandler,
    ImplResourceRequestHandler, ImplResponse, RequestHandler, ResourceRequestHandler, ResourceType,
    ResponseFilter, UrlrequestStatus, rc::RcImpl, sys,
};

use cef::{CefStringMultimap, ImplCallback, ReturnValue};

use crate::config::Config;
use crate::filter::DemoResponseFilter;
//
// RequestHandler
//

/// A custom implementation of `RequestHandler` for handling CEF browser requests.
///
/// This handler serves as the first point of contact for all requests in the browser.
/// It identifies XHR requests and applies specialized handling including cache prevention,
/// request monitoring, and delegation to appropriate resource handlers.
pub struct DemoRequestHandler {
    pub base: *mut RcImpl<sys::_cef_request_handler_t, Self>,
    pub config: Option<Config>,
}

impl DemoRequestHandler {
    /// Creates a new instance of `DemoRequestHandler`.
    ///
    /// # Returns
    /// A new `RequestHandler` instance wrapping the `DemoRequestHandler` implementation.
    ///
    pub(crate) fn new(config: Option<Config>) -> RequestHandler {
        RequestHandler::new(Self {
            base: std::ptr::null_mut(),
            config 
        })
    }
}

impl ImplRequestHandler for DemoRequestHandler {
    /// Provides a resource request handler for specific requests.
    ///
    /// This method is called by the CEF framework when it needs to determine how to handle
    /// a particular request. It analyzes the request to identify XHR requests and applies
    /// specialized handling for them.
    ///
    /// # Parameters
    /// - `_browser`: The browser instance initiating the request.
    /// - `_frame`: The frame within the browser that is making the request.
    /// - `_request`: The request object containing details about the HTTP request.
    /// - `_is_navigation`: Indicates if the request is a navigation (1) or not (0).
    /// - `_is_download`: Indicates if the request is a download (1) or not (0).
    /// - `_request_initiator`: The origin of the request (security domain).
    /// - `_disable_default_handling`: Optional flag to disable default handling.
    ///
    /// # Returns
    /// An optional `ResourceRequestHandler` instance for request processing,
    /// or `None` if default browser handling should be used.
    ///
    /// # Behavior
    /// - For XHR requests: Sets cache prevention headers and returns a specialized handler
    /// - For non-navigation, non-download requests: Returns a handler for monitoring
    /// - For all other requests: Returns None to use default browser handling
    fn get_resource_request_handler(
        &self,
        _browser: Option<&mut impl ImplBrowser>,
        _frame: Option<&mut impl ImplFrame>,
        _request: Option<&mut impl ImplRequest>,
        _is_navigation: ::std::os::raw::c_int,
        _is_download: ::std::os::raw::c_int,
        _request_initiator: Option<&CefString>,
        _disable_default_handling: Option<&mut ::std::os::raw::c_int>,
    ) -> Option<ResourceRequestHandler> {
        let request = _request.unwrap();

        // Check if its an XHR request
        if request.get_resource_type() == ResourceType::from(sys::cef_resource_type_t::RT_XHR) {
            let cache_control_str = CefString::from("Cache-Control");
            let pragma_str = CefString::from("Pragma");
            let expires_str = CefString::from("Expires");

            let cache_control_val = CefString::from("no-cache, no-store, must-revalidate");
            let pragma_val = CefString::from("no-cache");
            let expires_val = CefString::from("0");

            // Verify the header was set correctly
            let mut map = CefStringMultimap::new().unwrap();
            request.get_header_map(Some(&mut map));

            map.append(&cache_control_str, &cache_control_val);
            map.append(&pragma_str, &pragma_val);
            map.append(&expires_str, &expires_val);

            request.set_header_map(Some(&mut map));

            // let mut map2 = CefStringMultimap::new().unwrap();
            // request.get_header_map(Some(&mut map2));

            // eprintln!(">> {:?}", map2);

            // let url = CefStringUtf16::from(&request.get_url());
            // eprintln!(">>       | URL:  {:}", url.to_string().limit(120));
            // eprintln!(">>       | Mime: {:?}", request.get_resource_type());

            // if let Some(request_initiator) = _request_initiator {
                // eprintln!(">>       | Request initiator: {}", request_initiator);
            // }

            // eprintln!(">>       | Is navigation: {:}", _is_navigation);

            // eprintln!(">>       | Is download: {:}", _is_download);
            // eprintln!(">>       | Found XHR request");
            return Some(DemoResourceRequestHandler::new(self.config.clone()));
        }

        if _is_download == 0 && _is_navigation == 0 {
            Some(DemoResourceRequestHandler::new(self.config.clone()))
        } else {
            None
        }
    }

    /// Returns the raw pointer to the underlying CEF request handler.
    ///
    /// # Returns
    /// A raw pointer to the CEF request handler implementation.
    /// This is used by the CEF framework to call back into this implementation.
    fn get_raw(&self) -> *mut sys::_cef_request_handler_t {
        self.base as *mut sys::_cef_request_handler_t
    }
}

//
// ResourceHandler
//

/// A custom implementation of `ResourceRequestHandler` for handling resource requests.
///
/// This handler processes specific resource requests after they've been identified by
/// the `DemoRequestHandler`. It has the ability to:
/// - Modify requests before they are sent
/// - Process responses after they are received
/// - Apply filters to response content
/// - Track completion of resource loading
pub struct DemoResourceRequestHandler {
    pub base: *mut RcImpl<sys::_cef_resource_request_handler_t, Self>,
    pub config: Option<Config>,
}

impl DemoResourceRequestHandler {
    /// Creates a new instance of `DemoResourceRequestHandler`.
    ///
    /// # Returns
    /// A new `ResourceRequestHandler` instance wrapping the `DemoResourceRequestHandler` implementation.
    ///
    /// # Examples
    /// ```
    /// let resource_handler = DemoResourceRequestHandler::new();
    /// ```
    fn new(config: Option<Config>) -> ResourceRequestHandler {
        ResourceRequestHandler::new(Self {
            base: std::ptr::null_mut(),
            config
        })
    }
}

impl ImplResourceRequestHandler for DemoResourceRequestHandler {
    /// Called before a resource is loaded.
    ///
    /// This method allows examining and modification of request parameters before
    /// the request is actually sent to the server.
    ///
    /// # Parameters
    /// - `_browser`: The browser instance initiating the request.
    /// - `_frame`: The frame within the browser that is making the request.
    /// - `_request`: The request object that can be modified before sending.
    /// - `_callback`: A callback for asynchronous request handling.
    ///
    /// # Returns
    /// A `ReturnValue` indicating how the resource load should proceed:
    /// - `RV_CONTINUE`: Continue with the request.
    /// - `RV_CANCEL`: Cancel the request.
    /// - `RV_CONTINUE_ASYNC`: Handle the request asynchronously using the callback.
    fn on_before_resource_load(
        &self,
        _browser: Option<&mut impl ImplBrowser>,
        _frame: Option<&mut impl ImplFrame>,
        _request: Option<&mut impl ImplRequest>,
        _callback: Option<&mut impl ImplCallback>,
    ) -> ReturnValue {
        ReturnValue::from(cef_return_value_t::RV_CONTINUE)
    }

    /// Called when a resource response is received.
    ///
    /// This method allows examining the response headers and status before
    /// the response body is processed.
    ///
    /// # Parameters
    /// - `_browser`: The browser instance processing the response.
    /// - `_frame`: The frame within the browser that received the response.
    /// - `_request`: The request object that generated this response.
    /// - `_response`: The response object containing status, headers, etc.
    ///
    /// # Returns
    /// An integer indicating the result of the response handling:
    /// - `0`: Continue with default handling.
    /// - `1`: Continue without a response filter.
    fn on_resource_response(
        &self,
        _browser: Option<&mut impl ImplBrowser>,
        _frame: Option<&mut impl ImplFrame>,
        _request: Option<&mut impl ImplRequest>,
        _response: Option<&mut impl ImplResponse>,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    /// Provides a response filter for modifying resource responses.
    ///
    /// This method is called to determine if a filter should be applied
    /// to the response data before it's delivered to the renderer.
    ///
    /// # Parameters
    /// - `_browser`: The browser instance processing the response.
    /// - `_frame`: The frame within the browser that received the response.
    /// - `_request`: The request object that generated this response.
    /// - `_response`: The response object containing status, headers, etc.
    ///
    /// # Returns
    /// An optional `ResponseFilter` implementation that will process the response data.
    /// Returns `None` if no filtering is needed.
    fn get_resource_response_filter(
        &self,
        _browser: Option<&mut impl ImplBrowser>,
        _frame: Option<&mut impl ImplFrame>,
        _request: Option<&mut impl ImplRequest>,
        _response: Option<&mut impl ImplResponse>,
    ) -> Option<ResponseFilter> {
        let mut headers = CefStringMultimap::new().unwrap();

        if let Some(req) = _request.as_ref() {
            req.get_header_map(Some(&mut headers));
        }

        let url = {
            if let Some(req) = _request.as_ref() {
                let uri = req.get_url();
                CefString::from(&uri).to_string()
            } else {
                String::from("")
            }
        };

        Some(DemoResponseFilter::new(headers, self.config.clone(), url))
    }

    /// Called when a resource load is complete.
    ///
    /// This method provides notification about the completion status of a request,
    /// whether it was successful or failed.
    ///
    /// # Parameters
    /// - `_browser`: The browser instance that initiated the request.
    /// - `_frame`: The frame within the browser that made the request.
    /// - `_request`: The request object containing the original request details.
    /// - `_response`: The response object with final response details.
    /// - `_status`: The status of the URL request.
    /// - `_received_content_length`: The number of response content bytes actually received.
    fn on_resource_load_complete(
        &self,
        _browser: Option<&mut impl ImplBrowser>,
        _frame: Option<&mut impl ImplFrame>,
        _request: Option<&mut impl ImplRequest>,
        _response: Option<&mut impl ImplResponse>,
        _status: UrlrequestStatus,
        _received_content_length: i64,
    ) {
    }

    /// Returns the raw pointer to the underlying CEF resource request handler.
    ///
    /// # Returns
    /// A raw pointer to the CEF resource request handler implementation.
    /// This is used by the CEF framework to call back into this implementation.
    fn get_raw(&self) -> *mut sys::_cef_resource_request_handler_t {
        self.base as *mut sys::_cef_resource_request_handler_t
    }
}
