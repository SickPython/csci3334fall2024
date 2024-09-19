fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    
    let numbers: [i32; 10] = [2, 3, 5, 6, 7, 8, 9, 10, 12, 15];

    for &num in numbers.iter() {

        if is_even(num) {
            println!("{num} is even");
        } else {
            println!("{num} is odd");
        }

        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("The sum of all numbers in the array is: {sum}");

    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number in the array is: {largest}");
}