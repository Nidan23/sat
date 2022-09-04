# Version

 - Cargo 1.63.0 (fd9c4297c 2022-07-01)

# Requirements

 - Cargo 1.63
 - Free 8080 port
 - Cargo added 2 PATH

# Usage

 - Type `./run.bat` and test with Postman or whatever you want
 - You're welcome ;)

# Endpoints

 - localhost:8080/probabilityOfUnitInjectorFail
   - Query Params
     - VIN -> string
   - String -> probability
 - localhost:8080/calculateDieselUsageForDistance
   - Query Params
     - distance -> u32 / int
     - yearOfProduction -> u32 / int
     - fuelUsagePer100KM -> f32 / float
   - Response
     - String -> fuel usage