use std::io;
fn main() {
    
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
    


}
