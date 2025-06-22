use std::io;
fn main() {
    
    println!("Program for solving quadratic equation");
    
    let mut input_a: String = String::new();
    let mut input_b: String = String::new();
    
    let mut input_c: String = String::new();

    // ax^2 + bx + c = 0
    // D = b^2 - 4(a*c)
    println!("Enter a:");
    match io::stdin().read_line(&mut input_a) {
        Ok(_) => {},
        Err(e) => println!("Failed to gather information. 
        Error code: {}", e)
    }
    println!("Enter b:");
    match io::stdin().read_line(&mut input_b) {
        Ok(_) => {},
        Err(e) => println!("Failed to gather information. 
        Error code: {}", e)
    }
    println!("Enter c:");

    match io::stdin().read_line(&mut input_c) {
        Ok(_) => {},
        Err(e) => println!("Failed to gather information. 
        Error code: {}", e)
    }
    
    //Convert String to float
    let a: f64 = input_a.trim().parse().unwrap();
    let b: f64 = input_b.trim().parse().unwrap();
    let c: f64 = input_c.trim().parse().unwrap();
    
    //D-formula
    let d = b * b - 4.0 * a * c;
    println!("D = {}", d);

}
