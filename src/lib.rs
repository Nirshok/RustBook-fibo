
pub fn fibo_calc(numb: u64) -> u128 {
    match numb {
        // First two numbers of Fibonacchi is 0 and 1
        1 => 0,
        2 => 1,
        // Calculating all the others
        numb => {
            let mut count = numb;
            let mut first: u128 = 0;
            let mut second: u128 = 1;
            // Going until count is bigger than 2
            // Since first two numbers are already written
            while count > 2 {
                // Check if it can be stored, insert with 0 if it can't
                // And return to the inputting
                let fibo = first.checked_add(second).unwrap_or(4);
                if fibo == 4 {
                    println!("\nValue is too high to store!\n");
                    return fibo;
                } else {
                    // First number for the next iteration becomes current's one second
                    // Second number for the next iteration becomes the sum of previous
                    // First and Second numbers
                    // Decreasing count by 1
                    first = second;
                    second = fibo;
                    count -= 1;
                }
            }
            second
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let desired = 5;
        let calc = fibo_calc(desired);

        assert_eq!(calc, 3);
    }

    #[test]
    fn too_big() {
        let desired: u64 = 1000;
        let calc = fibo_calc(desired);

        assert_eq!(calc, 4);
    }
}