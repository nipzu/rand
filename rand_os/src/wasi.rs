// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for wasi

extern crate libc;

use rand_core::{Error, ErrorKind};
use super::OsRngImpl;

#[derive(Clone, Debug)]
pub struct OsRng;

impl OsRngImpl for OsRng {
    fn new() -> Result<OsRng, Error> { Ok(OsRng) }

    fn fill_chunk(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        let ret = unsafe { 
            libc::__wasi_random_get(dest.as_mut_ptr() as *mut libc::c_void, dest.len())
        };
        if ret == libc::__WASI_ESUCCESS {
            Ok(())
        } else {
            Err(Error::with_cause(
                ErrorKind::Unexpected,
                "__wasi_random_get failed",
                std::io::Error::from_raw_os_error(ret as i32),
            ))
        }
    }

    fn method_str(&self) -> &'static str { "__wasi_random_get" }
}
