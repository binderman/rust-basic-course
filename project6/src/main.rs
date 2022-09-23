fn calculate_salary(hours: f32, rate: f32) -> f32 {
    hours * rate
}

fn main() {
    println!("{}", calculate_salary(40.0, 20.0));
}