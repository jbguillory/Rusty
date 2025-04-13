use::std::io;

fn main() {
    println!("Enter your weight in (kg) loser!:");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).err();

    let weight = input.trim().parse().unwrap();

    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32{
    (weight /9.81) * 3.711
}
