use tempalte_lib::ffi;

#[test]
fn test_ffi_calculator() {
    unsafe {
        // 创建 Calculator 实例
        let calculator = ffi::calculator_new();

        // 测试加法
        assert_eq!(ffi::calculator_add(calculator, 2.0, 3.0), 5.0);

        // 测试减法
        assert_eq!(ffi::calculator_subtract(calculator, 5.0, 3.0), 2.0);

        // 测试乘法
        assert_eq!(ffi::calculator_multiply(calculator, 2.0, 3.0), 6.0);

        // 测试除法
        let mut error: *const i8 = std::ptr::null();
        assert_eq!(ffi::calculator_divide(calculator, 6.0, 3.0, &mut error), 2.0);
        assert!(error.is_null());

        assert_eq!(ffi::calculator_divide(calculator, 6.0, 0.0, &mut error), 0.0);
        assert!(!error.is_null());

        // 释放 Calculator 实例
        ffi::calculator_free(calculator);
    }
}