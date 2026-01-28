const FREEZING_POINT: f64=32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT)*5.0/9.0

}

//fn celsius_to_fahrenheit(c: f64)->f64{



fn main() {
    // 3a. Mutable Fahrenheit temperature
    let mut temp_f: f64 = 32.0;

    // 3b. Convert and print initial value
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{temp_f}°F = {temp_c:.2}°C");

    // 3c. Loop to print next 5 integer temperatures
    for _ in 0..5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{temp_f}°F = {temp_c:.2}°C");
    }
}
