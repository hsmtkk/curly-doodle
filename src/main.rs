use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight: f64 = input.trim().parse().unwrap();
    println!("Weight on Earth: {} kg", weight);

    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {} kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f64) -> f64{
    return weight / 9.81 * 3.711;
}
