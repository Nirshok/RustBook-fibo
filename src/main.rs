use std::io;
use fibo::*;
fn main() {
    println!("Enter desired number:\n");

    'outer: loop {
        let mut numb = String::new();
        // Reading input and storing it into numb
        io::stdin()
            .read_line(&mut numb)
            .expect("Failed to read the line.");
        // Parsing numb into u64, returning to the beginning if not possible
        let numb: u64 = match numb.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nError, input is not a number, too big or a negative\n");
                continue;
            }
        };
        // Calculating desired fibonachhi number
        // Returns 4 if it overflows
        let result = fibo_calc(numb);
        
        if result == 4 {
            continue 'outer;
        } else {
            println!("{numb}th number of Fibonacchi is {result}");
        }
        break;
    }
}
