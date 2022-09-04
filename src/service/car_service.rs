use rand::Rng;

pub fn calculate_fuel_consumption(distance: u32, fuel_consumption: f32) -> f32 {
    return ( distance as f32 / 100.0 ) * fuel_consumption
}

pub fn calculate_injector_fail() -> f32 {
    return rand::thread_rng().gen()
}

pub fn format_float(number: f32) -> String {
    format!("{:.2}", number)
}