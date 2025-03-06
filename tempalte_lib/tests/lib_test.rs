// 引入库 crate 中的 Calculator 结构体
use tempalte_lib::Calculator; // 假设 crate 名称是 my_lib

#[test]
fn test_calculator() {
    let calc = Calculator::new();

    // 测试加法
    assert_eq!(calc.add(2.0, 3.0), 5.0);

    // 测试减法
    assert_eq!(calc.subtract(5.0, 3.0), 2.0);

    // 测试乘法
    assert_eq!(calc.multiply(2.0, 3.0), 6.0);

    // 测试除法
    assert_eq!(calc.divide(6.0, 3.0), Ok(2.0));
    assert_eq!(calc.divide(6.0, 0.0), Err("除数不能为零"));
}