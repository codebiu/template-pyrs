use std::ffi::c_void;
use super::Calculator;

#[no_mangle]
pub extern "C" fn calculator_new() -> *mut c_void {
    let calculator = Box::new(Calculator::new());
    Box::into_raw(calculator) as *mut c_void
}

#[no_mangle]
pub extern "C" fn calculator_free(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr as *mut Calculator);
        }
    }
}

#[no_mangle]
pub extern "C" fn calculator_add(ptr: *mut c_void, a: f64, b: f64) -> f64 {
    let calculator = unsafe { &*(ptr as *mut Calculator) };
    calculator.add(a, b)
}

#[no_mangle]
pub extern "C" fn calculator_subtract(ptr: *mut c_void, a: f64, b: f64) -> f64 {
    let calculator = unsafe { &*(ptr as *mut Calculator) };
    calculator.subtract(a, b)
}

#[no_mangle]
pub extern "C" fn calculator_multiply(ptr: *mut c_void, a: f64, b: f64) -> f64 {
    let calculator = unsafe { &*(ptr as *mut Calculator) };
    calculator.multiply(a, b)
}

#[no_mangle]
pub extern "C" fn calculator_divide(ptr: *mut c_void, a: f64, b: f64, error: *mut *const i8) -> f64 {
    let calculator = unsafe { &*(ptr as *mut Calculator) };
    match calculator.divide(a, b) {
        Ok(result) => {
            unsafe { *error = std::ptr::null() };
            result
        }
        Err(err_msg) => {
            unsafe { *error = err_msg.as_ptr() as *const i8 };
            0.0
        }
    }
}