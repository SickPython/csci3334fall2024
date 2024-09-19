const F_FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - F_FREEZING_POINT) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (5.0 / 9.0) + F_FREEZING_POINT 
}

fn main() {

    let mut fahrenheit: f64 = 32.0;
    
    let celsius = fahrenheit_to_celsius(fahrenheit);
    
    println!("{} Fahrenheit is {} Celsius", fahrenheit, celsius);

    for idx in 1..=5 {
        let celsius = fahrenheit_to_celsius(fahrenheit);
        fahrenheit += 1.0;
        println!("{} Fahrenheit is {} Celsius", fahrenheit, celsius);
    }

}
