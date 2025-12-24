use itertools::{Itertools, Either};
use rand::prelude::IndexedRandom;
use lazy_static::lazy_static;
use serde::Deserialize;

use std::fmt;
use std::convert::From;

mod data_loader;

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct City {
    name: String,
    latitude: f64,
    longitude: f64,
    population: f64,
    country: String,
    province: String,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

lazy_static! {
    static ref DATA: Vec<City> = data_loader::load_data().unwrap();

    static ref NORTH_POLE: City = City {
        name: String::from("North Pole"),
        latitude: 90.0,
        longitude: 0.0,
        population: 0.0,
        country: String::from("North Pole"),
        province: String::from("North Pole"),
    };

    static ref SOUTH_POLE: City = City {
        name: String::from("South Pole"),
        latitude: -90.0,
        longitude: 0.0,
        population: 0.0,
        country: String::from("Antarctica"),
        province: String::from("South Pole"),
    };
}

const LATITUDE_TOLERANCE: f64 = 0.5;

const NUM_CITIES: usize = 10;
// The cities with the highest population may or may not include the current city;
// I want to be sure I'm always including the current city exactly once and last.
// So unconditionally remove the last sorted east/north and add on the current city
// instead.
const NUM_CITIES_LATITUDE: usize = 9;
// North Pole and South Pole are always included in the longitude text, but they
// aren't particularly interesting; I want to return 10 cities plus them.
const NUM_CITIES_LONGITUDE: usize = 11;

pub fn random_location() -> String {
    let mut rng = rand::rng();
    let city = DATA.choose(&mut rng).unwrap();

    location_text(city)
}

pub fn location_text(city: &City) -> String {
    format!("You are now in {}, {}, {}
{} {}
{}
{}",
        city,
        city.province,
        city.country,
        latitude_in_degrees(city.latitude),
        longitude_in_degrees(city.longitude),
        latitude_text(&city),
        longitude_text(&city),
    )
}

fn same_latitude(lat: f64) -> Vec<City> {
    DATA
        .iter()
        .cloned()
        .filter(|city| {
            city.latitude < lat + LATITUDE_TOLERANCE &&
            lat - LATITUDE_TOLERANCE < city.latitude
        })
        .collect()
}

fn opposite_longitude(long: f64) -> f64 {
    let mut opposite_long = 180.0 - long.abs();

    if long > 0.0 {
        opposite_long *= -1.0;
    };

    opposite_long
}

const LONGITUDE_TOLERANCE: f64 = 0.5;

fn same_longitude(long: f64) -> Vec<City> {
    let opposite_long = opposite_longitude(long);

    DATA
        .iter()
        .cloned()
        .filter(|city| {
            (city.longitude < long + LONGITUDE_TOLERANCE &&
            long - LONGITUDE_TOLERANCE < city.longitude) ||
            (city.longitude < opposite_long + LONGITUDE_TOLERANCE &&
            opposite_long - LONGITUDE_TOLERANCE < city.longitude)
        })
        .collect()
}

fn top_by_population(mut cities: Vec<City>) -> Vec<City> {
    cities.sort_by(|a, b| b.population.partial_cmp(&a.population).unwrap());
    cities.into_iter().take(NUM_CITIES).collect()
}

fn sort_easterly(mut cities: Vec<City>, start_long: f64) -> Vec<City> {
    cities.sort_by(|a, b| a.longitude.partial_cmp(&b.longitude).unwrap());

    let (mut west, mut east): (Vec<_>, Vec<_>) = cities
        .into_iter()
        .partition_map(|city| {
            if city.longitude <= start_long {
                Either::Left(city)
            } else {
                Either::Right(city)
            }
        });
    east.append(&mut west);
    east
}

fn sort_northerly(cities: Vec<City>, start_lat: f64, start_long: f64) -> Vec<City> {
    let start_long_negative = start_long < 0.0;

    let (mut same_side, mut opp_side): (Vec<_>, Vec<_>) = cities
        .into_iter()
        .partition_map(|city| {
            if start_long_negative {
                if city.longitude < 0.0 {
                   Either::Left(city)
                } else {
                    Either::Right(city)
                }
            } else {
                if city.longitude < 0.0 {
                   Either::Right(city)
                } else {
                    Either::Left(city)
                }
            }
        });

    same_side.sort_by(|a, b| a.latitude.partial_cmp(&b.latitude).unwrap());

    let (mut north, mut south): (Vec<_>, Vec<_>) = same_side
        .into_iter()
        .partition_map(|city| {
            if city.latitude > start_lat {
                Either::Left(city)
            } else {
                Either::Right(city)
            }
        });

    opp_side.sort_by(|a, b| b.latitude.partial_cmp(&a.latitude).unwrap());

    north.push(NORTH_POLE.clone());
    north.append(&mut opp_side);
    north.push(SOUTH_POLE.clone());
    north.append(&mut south);
    north
}

fn latitude_cities(latitude: f64, longitude: f64) -> Vec<City> {
    let cities = same_latitude(latitude);
    let cities = top_by_population(cities);
    let cities = sort_easterly(cities, longitude);
    cities
}

fn latitude_text(city: &City) -> String {
    let (latitude, longitude) = (city.latitude, city.longitude);
    format!("If you fly along this latitude in an easterly direction, you will look down on {}, {}.", latitude_cities(latitude, longitude).iter().take(NUM_CITIES_LATITUDE).join(", "), city)
}

fn longitude_cities(latitude: f64, longitude: f64) -> Vec<City> {
    let cities = same_longitude(longitude);
    let cities = top_by_population(cities);
    let cities = sort_northerly(cities, latitude, longitude);
    cities
}

fn longitude_text(city: &City) -> String {
    let (latitude, longitude) = (city.latitude, city.longitude);
    format!("If you fly along this longitude starting north, you will look down on {}, {}.", longitude_cities(latitude, longitude).iter().take(NUM_CITIES_LONGITUDE).join(", "), city)
}

fn decimal_to_degrees_minutes(coord: f64) -> (f64, f64) {
    (
        coord.abs().floor(),
        ((coord.abs() * 60.0) % 60.0).floor()
    )
}

fn latitude_in_degrees(coord: f64) -> String {
    let dir = if coord > 0.0 { "N" } else { "S" };
    let (deg, min) = decimal_to_degrees_minutes(coord);
    format!("{}°{}'{}", deg, min, dir)
}

fn longitude_in_degrees(coord: f64) -> String {
    let dir = if coord > 0.0 { "E" } else { "W" };
    let (deg, min) = decimal_to_degrees_minutes(coord);
    format!("{}°{}'{}", deg, min, dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn it_finds_cities_with_same_latitude() {
        let lat = 40.4299986;
        let cities = same_latitude(lat);
        let mut names: Vec<_> = cities.iter().map(|ref c| &c.name).collect();
        names.sort();
        assert_eq!(
            names,
            vec!["Adapazari", "Agdam", "Alexandroupoli", "Ali Bayramli", "Allentown", "Altoona", "Amasya", "Andijon", "Anxi", "Aomori", "Arcata", "Artashat", "Ashtarak", "Aveiro", "Baku", "Baotou", "Beaver Falls", "Berat", "Bilecik", "Bloomington", "Bolu", "Boulder", "Brindisi", "Burlington", "Bursa", "Canakkale", "Cankiri", "Canton", "Castello", "Changping", "Chosan", "Coimbra", "Columbus", "Corovode", "Corum", "Covilha", "Craig", "Dandong", "Datong", "Dunhuang", "Elko", "Erseke", "Eureka", "Fargona", "Fengzhen", "Fier", "Fort Collins", "Gadabay", "Ganca", "Gavarr", "Giresun", "Gjirokaster", "Goranboy", "Goycay", "Gramsh", "Grand Island", "Greeley", "Guadalajara", "Guarda", "Guliston", "Gumushane", "Gyumri", "Hachinohe", "Hanggin Houqi", "Harrisburg", "Hirosaki", "Hohhot", "Ijevan", "Izmit", "Jinxi", "Jizzax", "Johnstown", "Kars", "Katerini", "Kearney", "Khujand", "Kimchaek", "Kimhyonggwon", "Kirksville", "Kokomo", "Konibodom", "Korce", "Lafayette", "Lancaster", "Lecce", "Lima", "Lincoln", "Madrid", "Mansfield", "Marion", "McCook", "Muncie", "Naples", "Navoi", "New York", "Newark", "Olbia", "Olmaliq", "Osh", "Paterson", "Peoria", "Permet", "Philadelphia", "Pittsburgh", "Pogradec", "Polygyros", "Potenza", "Provo", "Qinhuangdao", "Qoqon", "Quincy", "Redding", "Sakarya", "Salerno", "Salt Lake City", "Sassari", "Sinuiju", "State College", "Sumqayt", "Taedong", "Taranto", "Tepelene", "Thessaloniki", "Tokat", "Trenton", "Turkmenbasy", "Urbana", "Vanadzor", "Vernal", "Viseu", "Vlore", "Wheeling", "Xuanhua", "Yerevan", "Yevlax", "Yingkow", "York", "Zanesville", "Zhangjiakou"]
        );
    }

    #[test]
    fn it_filters_to_ten_by_population() {
        let lat = 40.4299986;
        let cities = same_latitude(lat);

        let cities = top_by_population(cities);

        let names: Vec<_> = cities.iter().map(|ref c| &c.name).collect();

        assert_eq!(
            names,
            vec!["New York", "Philadelphia", "Madrid", "Baku", "Naples", "Pittsburgh", "Datong", "Bursa", "Jinxi", "Hohhot"]
        );
    }

    #[test]
    fn it_sorts_easterly() {
        let lat = 40.4299986;
        let long = -79.99998539;
        let cities = same_latitude(lat);
        let cities = top_by_population(cities);

        let cities = sort_easterly(cities, long);

        let names: Vec<_> = cities.iter().map(|ref c| &c.name).collect();

        assert_eq!(
            names,
            vec!["Philadelphia", "New York", "Madrid", "Naples", "Bursa", "Baku", "Hohhot", "Datong", "Jinxi", "Pittsburgh"]
        );
    }

    #[test]
    fn it_creates_latitude_text() {
        let city = City {
            name: String::from("Pittsburgh"),
            latitude: 40.4299986,
            longitude: -79.99998539,
            population: 0.0, // doesn't matter here
            country: String::from("United States of America"),
            province: String::from("Pennsylvania"),
        };

        let latitude_text = latitude_text(&city);

        assert_eq!(
            latitude_text,
            "If you fly along this latitude in an easterly direction, you will look down on Philadelphia, New York, Madrid, Naples, Bursa, Baku, Hohhot, Datong, Jinxi, Pittsburgh."
        );
    }

    #[test]
    fn it_creates_latitude_text_for_small_cities() {
        let city = City {
            name: String::from("Jaque"),
            latitude: 7.518958353,
            longitude: -78.16601465,
            population: 0.0, // doesn't matter here
            country: String::from("Panama"),
            province: String::from("Darién"),
        };

        let latitude_text = latitude_text(&city);

        assert_eq!(
            latitude_text,
            "If you fly along this latitude in an easterly direction, you will look down on Bucaramanga, Cucuta, Bouake, Abeokuta, Ibadan, Oyo, Ife, Ado Ekiti, Ikare, Jaque."
        );
    }

    #[test]
    fn it_finds_longitude_on_other_side_of_the_world() {
        let long = -79.99998539;
        assert_approx_eq!(
            opposite_longitude(long),
            100.00001461
        );

        let long = long * -1.0;
        assert_approx_eq!(
            opposite_longitude(long),
            -100.00001461
        );
    }

    #[test]
    fn it_finds_cities_with_same_longitude() {
        let long = -79.99998539;
        let cities = same_longitude(long);
        let mut names: Vec<_> = cities.iter().map(|ref c| &c.name).collect();
        names.sort();
        assert_eq!(
            names,
            vec!["Alor Setar", "Ang Thong", "Babahoyo", "Balboa", "Ban Houayxay", "Barrie", "Beaver Falls", "Blacksburg", "Bukittinggi", "Butterworth", "Chainat", "Charleston", "Chiang Rai", "Chiclayo", "Chitre", "Chone", "Chulucanas", "Cienfuegos", "Clarksburg", "Cobalt", "Colon", "Coral Gables", "Coral Springs", "Dali", "Erie", "Esmeraldas", "Ferrenafe", "Florence", "Fort Lauderdale", "Fort Pierce", "George Town", "Greensboro", "Guayaquil", "Hamilton", "Hat Yai", "Homestead", "Hua Hin", "Kamphaeng Phet", "Kanchanaburi", "Kangar", "Las Tablas", "Lijiang", "Macara", "Machala", "Miami", "Miami Beach", "Milagro", "Morgantown", "Moron", "Motupe", "Muisne", "Nakhon Pathom", "Nakhon Sawan", "Nakhon Si Thammarat", "New Liskeard", "Nonthaburi", "Olmos", "Orangeville", "Pacasmayo", "Padang", "Padangpanjang", "Panama City", "Parry Sound", "Penonome", "Phatthalung", "Phayao", "Phetchaburi", "Phichit", "Phitsanulok", "Phrae", "Pimentel", "Pinas", "Pittsburgh", "Placetas", "Portoviejo", "Prachuap Khiri Khan", "Ratchaburi", "Roanoke", "Sagua la Grande", "Salisbury", "Samut Sakhon", "Samut Songkhram", "Santa Clara", "Satun", "Sing Buri", "Sukhothai", "Sumter", "Sungai Petani", "Supham Buri", "Thung Song", "Trang", "Tumbes", "Tura", "Uthai Thani", "Uttaradit", "Vero Beach", "West Palm Beach", "White Sulphur Springs", "Winston-Salem", "Zhangye"]
        );
    }

    #[test]
    fn it_filters_longitude_cities_to_ten_by_population() {
        let long = -79.99998539;
        let cities = same_longitude(long);

        let cities = top_by_population(cities);

        let names: Vec<_> = cities.iter().map(|ref c| &c.name).collect();

        assert_eq!(
            names,
            vec!["Miami", "Guayaquil", "George Town", "Pittsburgh", "Fort Lauderdale", "Padang", "Panama City", "West Palm Beach", "Hamilton", "Chiclayo"]
        );
    }

    #[test]
    fn it_sorts_northerly() {
        let lat = 40.4299986;
        let long = -79.99998539;
        let cities = same_longitude(long);
        let cities = top_by_population(cities);

        let cities = sort_northerly(cities, lat, long);

        let names: Vec<_> = cities.iter().map(|ref c| &c.name).collect();

        assert_eq!(
            names,
            vec!["Hamilton", "North Pole", "George Town", "Padang", "South Pole", "Chiclayo", "Guayaquil", "Panama City", "Miami", "Fort Lauderdale", "West Palm Beach", "Pittsburgh"]
        );
    }

    #[test]
    fn it_creates_longitude_text() {
        let city = City {
            name: String::from("Pittsburgh"),
            latitude: 40.4299986,
            longitude: -79.99998539,
            population: 0.0, // doesn't matter here
            country: String::from("United States of America"),
            province: String::from("Pennsylvania"),
        };

        let longitude_text = longitude_text(&city);

        assert_eq!(
            longitude_text,
            "If you fly along this longitude starting north, you will look down on Hamilton, North Pole, George Town, Padang, South Pole, Chiclayo, Guayaquil, Panama City, Miami, Fort Lauderdale, West Palm Beach, Pittsburgh."
        );
    }

    #[test]
    fn it_creates_longitude_text_for_small_cities() {
        let city = City {
            name: String::from("Jaque"),
            latitude: 7.518958353,
            longitude: -78.16601465,
            population: 0.0, // doesn't matter here
            country: String::from("Panama"),
            province: String::from("Darién"),
        };

        let longitude_text = longitude_text(&city);

        assert_eq!(
            longitude_text,
            "If you fly along this longitude starting north, you will look down on Raleigh, North Pole, Xining, Panzhihua, Kota Baharu, Kuala Lumpur, Shah Alam, Kelang, Malacca, Pekanbaru, South Pole, Jaque."
        );
    }

    #[test]
    fn it_creates_full_text() {
        let city = City {
            name: String::from("Pittsburgh"),
            latitude: 40.4299986,
            longitude: -79.99998539,
            population: 0.0, // doesn't matter here
            country: String::from("United States of America"),
            province: String::from("Pennsylvania"),
        };

        let text = location_text(&city);

        assert_eq!(
            text,
"You are now in Pittsburgh, Pennsylvania, United States of America
40°25'N 79°59'W
If you fly along this latitude in an easterly direction, you will look down on Philadelphia, New York, Madrid, Naples, Bursa, Baku, Hohhot, Datong, Jinxi, Pittsburgh.
If you fly along this longitude starting north, you will look down on Hamilton, North Pole, George Town, Padang, South Pole, Chiclayo, Guayaquil, Panama City, Miami, Fort Lauderdale, West Palm Beach, Pittsburgh."
        );
    }

    #[test]
    fn it_creates_full_text_for_small_cities() {
        let city = City {
            name: String::from("Jaque"),
            latitude: 7.518958353,
            longitude: -78.16601465,
            population: 0.0, // doesn't matter here
            country: String::from("Panama"),
            province: String::from("Darién"),
        };

        let text = location_text(&city);

        assert_eq!(
            text,
"You are now in Jaque, Darién, Panama
7°31'N 78°9'W
If you fly along this latitude in an easterly direction, you will look down on Bucaramanga, Cucuta, Bouake, Abeokuta, Ibadan, Oyo, Ife, Ado Ekiti, Ikare, Jaque.
If you fly along this longitude starting north, you will look down on Raleigh, North Pole, Xining, Panzhihua, Kota Baharu, Kuala Lumpur, Shah Alam, Kelang, Malacca, Pekanbaru, South Pole, Jaque."
        );
    }
}