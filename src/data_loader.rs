use serde::Deserialize;
use std::path::Path;
use csv::ReaderBuilder;
use std::fs::File;
use std::convert::From;

use super::City;

#[derive(Deserialize)]
struct Record {
    city: String,
    _city_ascii: String,
    latitude: f64,
    longitude: f64,
    population: f64,
    country: String,
    _iso2: String,
    _iso3: String,
    province: String,
}

impl From<Record> for City {
    fn from(record: Record) -> City {
        City {
            name: record.city,
            latitude: record.latitude,
            longitude: record.longitude,
            population: record.population,
            country: record.country,
            province: record.province,
        }
    }
}

pub fn load_data() -> Result<Vec<City>, Box<dyn std::error::Error>> {
    let file_path = Path::new("./simplemaps-worldcities-basic.csv");
    let file = File::open(file_path)
        .map_err(|e| format!("Failed to open file {}: {}", file_path.display(), e))?;

    // Initialize CSV reader
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    // Print the number of records to be processed
    println!("=== Reading data from file: {:?} ===", file_path);

    let mut cities: Vec<City> = Vec::new();

    // Deserialize the CSV rows
    for (index, result) in rdr.deserialize::<Record>().enumerate() {
        match result {
            Ok(record) => {
                // Successfully deserialized, convert into City and push to the vector
                let city: City = record.into();
                cities.push(city);
                if index % 1000 == 0 {
                    println!("Processed {} records...", index + 1); // Print progress
                }
            }
            Err(e) => {
                // Error during deserialization, print which record failed
                eprintln!("Error deserializing row {}: {:?}", index, e);
            }
        }
    }

    // Check if we have data
    if cities.is_empty() {
        return Err("No valid city data found in CSV file.".into());
    }

    println!(" === Successfully loaded {} cities ===", cities.len());
    Ok(cities)
}