use std::{cmp::Ordering, fs::File, io::BufRead, io::BufReader};

fn main() {
    let result = run("input.txt");
    println!("result = {result}");
}

#[derive(Debug)]
struct FarmingRange {
    source: usize,
    length: usize,
    conversion_offset: i64,
}

impl FarmingRange {
    fn new(str_description: String) -> Self {
        let split: Vec<&str> = str_description.split_whitespace().collect();
        let destination: i64 = split[0]
            .parse()
            .expect(&format!("Unable to unwrap destination {}", split[0]));
        let source = split[1]
            .parse()
            .expect(&format!("Unable to unwrap source {}", split[1]));
        let length = split[2]
            .parse()
            .expect(&format!("Unable to unwrap length {}", split[2]));
        Self {
            source,
            length,
            conversion_offset: (destination - source as i64),
        }
    }

    fn process_source(&self, value: usize) -> usize {
        (value as i64 + self.conversion_offset) as usize
    }
}

#[derive(Debug)]
struct FarmingMap {
    ranges: Vec<FarmingRange>,
}

impl FarmingMap {
    fn new() -> Self {
        Self { ranges: vec![] }
    }
}

#[derive(Debug)]
enum CollectingData {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

fn run(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();

    let mut collection_state = CollectingData::Seeds;
    let mut seed_vec: Vec<usize> = vec![];
    let mut seed_to_soil = FarmingMap::new();
    let mut soil_to_fertilizer = FarmingMap::new();
    let mut fertilizer_to_water = FarmingMap::new();
    let mut water_to_light = FarmingMap::new();
    let mut light_to_temp = FarmingMap::new();
    let mut temp_to_humidity = FarmingMap::new();
    let mut humidity_to_location = FarmingMap::new();
    for line in lines {
        let line: String = line.unwrap();
        if line.is_empty() {
            continue;
        }
        if line.contains("map:") {
            let fragment = line.split(" map:").next().unwrap();
            match fragment {
                "seeds" => collection_state = CollectingData::Seeds,
                "seed-to-soil" => collection_state = CollectingData::SeedToSoil,
                "soil-to-fertilizer" => collection_state = CollectingData::SoilToFertilizer,
                "fertilizer-to-water" => collection_state = CollectingData::FertilizerToWater,
                "water-to-light" => collection_state = CollectingData::WaterToLight,
                "light-to-temperature" => collection_state = CollectingData::LightToTemperature,
                "temperature-to-humidity" => {
                    collection_state = CollectingData::TemperatureToHumidity
                }
                "humidity-to-location" => collection_state = CollectingData::HumidityToLocation,
                &_ => panic!("Received unexpected line {fragment}"),
            }
            continue;
        }
        match collection_state {
            CollectingData::Seeds => {
                if line.contains("seeds: ") {
                    let seedlist = line.replace("seeds: ", " ");
                    let pairs_list: Vec<usize> = seedlist
                        .split_whitespace()
                        .map(|s| s.parse().expect(&format!("Unable to unwrap {s}")))
                        .collect();
                    for pair in pairs_list.chunks(2) {
                        for num in 0..pair[1] {
                            seed_vec.push(pair[0] + num);
                        }
                    }
                }
            }
            CollectingData::SeedToSoil => seed_to_soil.ranges.push(FarmingRange::new(line)),
            CollectingData::SoilToFertilizer => {
                soil_to_fertilizer.ranges.push(FarmingRange::new(line))
            }
            CollectingData::FertilizerToWater => {
                fertilizer_to_water.ranges.push(FarmingRange::new(line))
            }
            CollectingData::WaterToLight => water_to_light.ranges.push(FarmingRange::new(line)),
            CollectingData::LightToTemperature => {
                light_to_temp.ranges.push(FarmingRange::new(line))
            }
            CollectingData::TemperatureToHumidity => {
                temp_to_humidity.ranges.push(FarmingRange::new(line))
            }
            CollectingData::HumidityToLocation => {
                humidity_to_location.ranges.push(FarmingRange::new(line))
            }
        }
    }
    let mut maps: [FarmingMap; 7] = [
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humidity,
        humidity_to_location,
    ];
    for map in maps.iter_mut() {
        map.ranges
            .sort_by(|a, b| a.source.partial_cmp(&b.source).unwrap());
    }
    let mut seed_values: Vec<usize> = vec![];
    for seed in seed_vec {
        let mut seed_value = seed;
        for map in &maps {
            if let Ok(idx) = map.ranges.binary_search_by(|range| {
                if range.source > seed_value {
                    Ordering::Greater
                } else if (range.source + range.length) <= seed_value {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }) {
                seed_value = map.ranges[idx].process_source(seed_value);
            }
        }
        seed_values.push(seed_value);
    }
    seed_values.sort();
    seed_values[0]
}

#[test]
fn sample_test() {
    // assert_eq!(run("sample_input.txt"), 35);
    assert_eq!(run("sample_input.txt"), 46);
}
