fn main() {
    let fahrenheit = 100.0;
    let celsius = fahrenheit_to_celsius(fahrenheit);

    println!("{fahrenheit}°F = {celsius:.2}°C");
}

fn fahrenheit_to_celsius(celsius: f32) -> f32 {
    celsius * 9.0 / 5.0 + 32.0
}