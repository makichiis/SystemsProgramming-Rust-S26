fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0) 
}
// (32°F − 32) × 5/9 = 0°C
fn celsius_to_fahrenheit(f: f64) -> f64 {
    (f * 9.0/5.0) + 32.0
}
// (0°C × 9/5) + 32 = 32°F

fn main() {
    let mut temperature = 32 as f64;
    let temperature_c = fahrenheit_to_celsius(temperature);
    println!("{}F -> {}C", temperature, temperature_c);

    for x in temperature as usize+1..temperature as usize + 6 {
        println!("{}F -> {}C", x, fahrenheit_to_celsius(x as f64));
    }
}
