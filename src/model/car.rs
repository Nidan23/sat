use serde::Deserialize;

#[derive(Deserialize)]
pub struct FuelConsumption {
    pub distance: u32,
    pub yearOfProduction: u32,
    pub fuelUsagePer100KM: f32
}

#[derive(Deserialize)]
pub struct EngineFail {
    pub VIN: String
}