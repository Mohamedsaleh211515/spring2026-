
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn ass1() {
    let mut temp_f: f64 = 32.0;

    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{temp_f}°F = {temp_c:.2}°C");

    for _ in 0..5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{temp_f}°F = {temp_c:.2}°C");
    }
}


fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn ass2() {
    let numbers: [i32; 10] = [12, 7, 9, 15, 22, 30, 4, 18, 5, 11];

    for num in numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{num}: FizzBuzz");
        } else if num % 3 == 0 {
            println!("{num}: Fizz");
        } else if num % 5 == 0 {
            println!("{num}: Buzz");
        } else if is_even(num) {
            println!("{num}: Even");
        } else {
            println!("{num}: Odd");
        }
    }

    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("Sum of numbers: {sum}");

    let mut largest = numbers[0];

    for num in numbers {
        if num > largest {
            largest = num;
        }
    }

    println!("Largest number: {largest}");
}


fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn ass3() {
    let secret = 7;
    let mut guesses = 0;
    let mut guess = 0;

    loop {
        guess += 1;
        guesses += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {guess} is correct!");
            break;
        } else if result == 1 {
            println!("Guess {guess} is too high.");
        } else {
            println!("Guess {guess} is too low.");
        }
    }

    println!("It took {guesses} guesses.");
}


fn main() {
    ass1();
    ass2();
    ass3();
}
