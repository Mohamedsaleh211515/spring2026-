fn ass1(){
   fn assignment1() {
    let mut temp_f: i32 = 32;

    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{temp_f}°F = {temp_c:.2}°C");

    for _ in 0..5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{temp_f}°F = {temp_c:.2}°C");
    }
   }
}

fn ass2(){
    println!("Run results")
}

fn ass3(){
    println!("Run results")
}

fn main(){
    fn ass1();
}