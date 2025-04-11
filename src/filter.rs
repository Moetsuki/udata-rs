#![allow(clippy::new_ret_no_self)]
#![allow(clippy::option_map_unit_fn)]
use cef::CefStringMultimap;
use cef::ImplResponseFilter;
use cef::ResponseFilter;
use cef::ResponseFilterStatus;
use cef::rc::RcImpl;
use cef::sys;
use cef::sys::cef_response_filter_status_t::{
    RESPONSE_FILTER_DONE, RESPONSE_FILTER_NEED_MORE_DATA,
};
use std::sync::{Arc, Mutex};

use crate::config::Config;

//
// ResponseFilter
//

/// A custom implementation of `ResponseFilter` for filtering and modifying response data.
///
/// This filter allows examining and potentially modifying HTTP response data before
/// it's processed by the browser. It can be used to:
/// - Inspect response contents
/// - Modify response data
/// - Collect metrics about the response
/// - Transform response formats
///
/// The filter maintains an internal buffer for accumulating data if needed.
pub struct DemoResponseFilter {
    /// Raw pointer to the CEF response filter implementation
    pub object: *mut RcImpl<sys::_cef_response_filter_t, Self>,
    /// Thread-safe buffer for storing response data during processing
    pub buffer: Arc<Mutex<Vec<u8>>>,
    /// Headers from the original request
    pub request_headers: CefStringMultimap,
    /// URL of the request being processed
    pub url: String,
    /// UUID of the request
    pub uuid: uuid::Uuid,
    /// General configuration
    pub config: Option<Config>,
}

impl DemoResponseFilter {
    /// Creates a new instance of `DemoResponseFilter`.
    ///
    /// # Parameters
    /// - `request_headers`: The headers of the original request, useful for context.
    /// - `url`: The URL of the request, used for logging and conditional processing.
    ///
    /// # Returns
    /// A new `ResponseFilter` instance wrapping the `DemoResponseFilter` implementation.
    /// ```
    pub fn new(request_headers: CefStringMultimap, config: Option<Config>, url: String) -> ResponseFilter {
        ResponseFilter::new(Self {
            object: std::ptr::null_mut(),
            buffer: Arc::new(Mutex::new(Vec::new())),
            request_headers,
            url,
            uuid: uuid::Uuid::new_v4(),
            config,
        })
    }
}

impl ImplResponseFilter for DemoResponseFilter {
    /// Initializes the filter.
    ///
    /// This method is called when the filter is first created and before any data processing.
    /// It should prepare the filter for data processing.
    ///
    /// # Returns
    /// An integer where:
    /// - `1` indicates successful initialization
    /// - `0` indicates initialization failure
    fn init_filter(&self) -> ::std::os::raw::c_int {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
        1 // Return true to indicate success
    }

    /// Filters the response data.
    ///
    /// This method is called for each chunk of data received from the network.
    /// It can examine and modify the data before it's passed to the renderer.
    ///
    /// # Parameters
    /// - `_data_in`: Optional buffer containing input data to filter.
    /// - `_data_in_read`: Output parameter that will contain the number of bytes read from `_data_in`.
    /// - `_data_out`: Optional buffer to write the filtered output data.
    /// - `_data_out_written`: Output parameter that will contain the number of bytes written to `_data_out`.
    ///
    /// # Returns
    /// A `ResponseFilterStatus` value:
    /// - `RESPONSE_FILTER_DONE`: All data has been processed successfully.
    /// - `RESPONSE_FILTER_NEED_MORE_DATA`: The filter needs more data to complete.
    /// - `RESPONSE_FILTER_ERROR`: An error occurred during filtering.
    ///
    /// # Notes
    /// - If `_data_in` is null, it indicates that no more data is available for filtering.
    /// - If `_data_out` is null, the filter should process `_data_in` and set `_data_in_read`
    ///   to the number of bytes consumed, but not produce any output.
    fn filter(
        &self,
        _data_in: Option<&mut Vec<u8>>,
        _data_in_read: Option<&mut usize>,
        _data_out: Option<&mut Vec<u8>>,
        _data_out_written: Option<&mut usize>,
    ) -> ResponseFilterStatus {
        // eprintln!(">>       +------+ FILTER");
        // eprintln!(">>              | URL:  {:}", self.url.limit(120));

        let mut binding = 0;
        let data_in_read = _data_in_read.unwrap_or(&mut binding);
        let mut binding2 = 0;
        let data_out_written = _data_out_written.unwrap_or(&mut binding2);
        *data_out_written = 0;

        if _data_in.is_none() {
            return ResponseFilterStatus::from(RESPONSE_FILTER_DONE);
        }

        let data_in = _data_in.unwrap();
        // eprintln!("data_in_size = {}", data_in.len());

        // If there's no output buffer, mark all input as read
        // This is a special CEF case we need to handle
        if _data_out.is_none() {
            *data_in_read = data_in.len();
            eprintln!("No output buffer, consuming all input: {}", *data_in_read);
            return ResponseFilterStatus::from(RESPONSE_FILTER_NEED_MORE_DATA);
        }

        let data_out = _data_out.unwrap();

        // Sanity
        let bytes_to_copy = std::cmp::min(data_in.len(), data_out.len());
        if bytes_to_copy > 0 {
            data_out[..bytes_to_copy].copy_from_slice(&data_in[..bytes_to_copy]);
            *data_out_written = bytes_to_copy;
        }

        // Mark how much input data we processed
        *data_in_read = bytes_to_copy;

        // eprintln!("data_out_written = {}", *data_out_written);
        // eprintln!("data_in_read = {}", *data_in_read);

        self.config.as_ref().map(|config| {
            config.host.iter().find(|host| {
                self.url.contains(&host.xhr)  
            }).map(|host| {
                eprintln!("Host match: {}", host.xhr);
                let buffer = data_in.clone();
                eprintln!("{}\n\n\t<<- -->\n\n", String::from_utf8(buffer).unwrap_or(String::from("")));
            });
        });

        if bytes_to_copy == data_in.len() {
            ResponseFilterStatus::from(RESPONSE_FILTER_DONE)
        } else {
            ResponseFilterStatus::from(RESPONSE_FILTER_NEED_MORE_DATA)
        }
    }

    /// Returns the raw pointer to the underlying CEF response filter.
    ///
    /// # Returns
    /// A raw pointer to the CEF response filter implementation.
    /// This is used by the CEF framework to call back into this implementation.
    fn get_raw(&self) -> *mut sys::_cef_response_filter_t {
        self.object as *mut sys::_cef_response_filter_t
    }
}
