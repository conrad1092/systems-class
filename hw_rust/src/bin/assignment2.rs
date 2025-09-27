// Assignment 2: Number Analyzer
// Analyzes an array of numbers for even/odd and FizzBuzz conditions.

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let nums: [i32; 10] = [12, 3, 5, 15, 8, 7, 30, 1, 2, 9];

    for &n in nums.iter() {
        if n % 15 == 0 {
            println!("{n}: FizzBuzz");
        } else if n % 3 == 0 {
            println!("{n}: Fizz");
        } else if n % 5 == 0 {
            println!("{n}: Buzz");
        } else if is_even(n) {
            println!("{n}: even");
        } else {
            println!("{n}: odd");
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < nums.len() {
        sum += nums[i];
        i += 1;
    }
    println!("Sum of all numbers: {sum}");

    let mut max_val = nums[0];
    let mut j = 1;
    loop {
        if j >= nums.len() {
            break;
        }
        if nums[j] > max_val {
            max_val = nums[j];
        }
        j += 1;
    }
    println!("Largest number: {max_val}");
}
