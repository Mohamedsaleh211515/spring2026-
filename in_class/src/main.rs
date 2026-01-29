/*const FREEZING_POINT: f64=32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT)*5.0/9.0

}

//fn celsius_to_fahrenheit(c: f64)->f64{



fn main() {
    let mut temp_f: f64 = 32.0;

    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{temp_f}°F = {temp_c:.2}°C");

    for _ in 0..5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{temp_f}°F = {temp_c:.2}°C");
    }
}

*/



fn get_rgb(char c) ->(u8,u8,u8){
    if c == 'R'{
        return(255,0,0);
    }
    if c == 'G'{
        return(0,255,0);
    }
    else {
        return (0,0,255);
    }
}
fn main(){
    // we are going to accept a letter like RGB
    // and we should return 
    // RED tuple (255,0,0)
    // GREEN tuple (0,255,0)
    // BLUE tuple (0,0,255)

    // write a function which accepts char 'r','g,'b'
    // and return above specified tuple

   fn get_rgb;

    
    
}


