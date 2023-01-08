use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut inputted_weight = String::new();
    _ = io::stdin().read_line(&mut inputted_weight).unwrap();

    let weight_number: f32 = inputted_weight.trim().parse().unwrap();

    let mars = calculate_wight_mars(weight_number).round();
    println!("Wight on mars: {}kg", mars);
}

fn calculate_wight_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
