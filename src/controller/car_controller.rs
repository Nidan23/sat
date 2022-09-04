use actix_web::{get, HttpResponse, Responder, web};
use crate::model::car::{EngineFail, FuelConsumption};
use crate::service::car_service::{calculate_fuel_consumption, format_float, calculate_injector_fail};

#[get("/calculateDieselUsageForDistance")]
pub async fn calculate_diesel_usage_for_distance(data: web::Query<FuelConsumption>) -> impl Responder {
    HttpResponse::Ok().body(calculate_fuel_consumption(data.distance, data.fuelUsagePer100KM).to_string())
}

#[get("/probabilityOfUnitInjectorFail")]
pub async fn probability_of_unit_injector_fail(_data: web::Query<EngineFail>) -> impl Responder {
    HttpResponse::Ok().body(format_float(calculate_injector_fail()))
}