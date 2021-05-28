use std::prelude::v1::*;

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

use std::borrow::Cow;

mod ffi_cow;
pub use ffi_cow::fficow_free;
pub use ffi_cow::{FFICow, FFIStr, FFIString};

use crate::encoding;

#[no_mangle]
pub unsafe extern "C" fn encoding_encode_s(s: *const c_char, len: usize) -> *const FFICow {
    if s.is_null() {
        eprintln!("encoding_encode: got a NULL s: {:?}", s);
        return core::ptr::null();
    }
    let cow = if len == 0 {
        let s = unsafe { CStr::from_ptr(s) }.to_string_lossy();
        encoding::encode(s.as_ref()).into()
    } else {
        let s = unsafe { std::slice::from_raw_parts_mut(s as *mut _, len) };
        encoding::encode(s).into()
    };

    eprintln!("retval ffi_cow {:?}", cow);

    Box::into_raw(Box::new(cow)) as *const _
}

#[no_mangle]
pub unsafe extern "C" fn encoding_encode<'a>(
    s: *const c_char,
    buf: *mut c_char,
    bufcap_ptr: *mut usize,
) -> c_int {
    use std::convert::TryFrom;

    if s.is_null() || buf.is_null() || bufcap_ptr.is_null() {
        eprintln!(
            "encoding_encode: got a NULL s: {:?}, buf: {:?}, bufcap_ptr: {:?}",
            s, buf, bufcap_ptr
        );
        return c_int::from(-1);
    }

    eprintln!(
        "encoding_encode: ptrs: s: {:?}, buf: {:?}, bufcap_ptr: {:?}",
        s, buf, bufcap_ptr
    );

    let cap = unsafe { *bufcap_ptr };

    let s = unsafe { std::slice::from_raw_parts_mut(buf as *mut _, *bufcap_ptr) };

    eprintln!(
        "encoding_encode: guard ok, bufcap {}, strlen: {}",
        cap,
        s.len()
    );
    if s.len() > cap {
        eprintln!(
            "encoding_encode: required {}, got buf capacity {}",
            s.len(),
            cap
        );
        return c_int::from(-1);
    }

    eprintln!("encoding_encode: encoding");
    let res = encoding::encode(s);

    let l = res.len();
    unsafe { *bufcap_ptr = l + 1 };
    eprintln!(
        "encoding_encode: encoded (len {}/{}): {}",
        l,
        cap,
        res.as_ref()
    );

    if l >= cap {
        eprintln!("encoding_encode: required {}, got capacity {}", l, cap);
        return c_int::from(-1);
    }

    let l = match isize::try_from(l) {
        Ok(l) => l,
        Err(_) => return c_int::from(-1),
    };

    let newbuf = if let Cow::Owned(r) = res {
        r.as_ptr()
    } else {
        s.as_ptr()
    };

    eprintln!("encoding_encode: copying buffer");
    unsafe {
        core::ptr::copy(newbuf, buf as *mut _, l as usize);
        *buf.offset(l) = c_char::from(0);
    }
    eprintln!("encoding_encode: done");

    c_int::from(0)
}
