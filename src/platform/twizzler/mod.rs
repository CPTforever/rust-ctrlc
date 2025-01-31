// Copyright (c) 2017 CtrlC developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::io;
use std::ptr;


/// Platform specific error type
pub type Error = io::Error;

unsafe extern "system" fn os_handler(_: u32) -> BOOL {
    FALSE
}

/// Register os signal handler.
///
/// Must be called before calling [`block_ctrl_c()`](fn.block_ctrl_c.html)
/// and should only be called once.
///
/// # Errors
/// Will return an error if a system error occurred.
///
#[inline]
pub unsafe fn init_os_handler(_overwrite: bool) -> Result<(), Error> {
    return Err(Error::new(
        ErrorKind::Unsupported,
        "Tell Ashley to complete this",
    )); 
}

/// Blocks until a Ctrl-C signal is received.
///
/// Must be called after calling [`init_os_handler()`](fn.init_os_handler.html).
///
/// # Errors
/// Will return an error if a system error occurred.
///
#[inline]
pub unsafe fn block_ctrl_c() -> Result<(), Error> {
    return Err(Error::new(
        ErrorKind::Unsupported,
        "Tell Ashley to complete this",
    )); 
}
