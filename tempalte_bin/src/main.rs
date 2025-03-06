use tempalte_lib::Calculator;

fn main() {
    let calculator = Calculator::new();
    
    // 演示加法
    let sum = calculator.add(10.5, 5.5);
    println!("10.5 + 5.5 = {}", sum);
    
    // 演示减法
    let difference = calculator.subtract(10.0, 3.0);
    println!("10.0 - 3.0 = {}", difference);
    
    // 演示乘法
    let product = calculator.multiply(4.0, 2.5);
    println!("4.0 * 2.5 = {}", product);
    
    // 演示除法
    match calculator.divide(15.0, 3.0) {
        Ok(quotient) => println!("15.0 / 3.0 = {}", quotient),
        Err(e) => println!("错误: {}", e)
    }
    
    // 演示除以零的错误处理
    match calculator.divide(10.0, 0.0) {
        Ok(quotient) => println!("10.0 / 0.0 = {}", quotient),
        Err(e) => println!("错误: {}", e)
    }
}
