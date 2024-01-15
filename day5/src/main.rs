use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let result = run("input.txt");
    println!("result = {result}");
}

struct FarmingRange {
    destination: usize,
    source: usize,
    length: usize,
}

impl FarmingRange {
    fn new(str_description: String) -> Self {
        let split: Vec<&str> = str_description.split_whitespace().collect();
        Self {
            destination: split[0]
                .parse()
                .expect(&format!("Unable to unwrap destination {}", split[0])),
            source: split[1]
                .parse()
                .expect(&format!("Unable to unwrap source {}", split[1])),
            length: split[2]
                .parse()
                .expect(&format!("Unable to unwrap length {}", split[2])),
        }
    }

    fn source_in_range(&self, value: usize) -> bool {
        value >= self.source && value < self.source + self.length
    }

    fn process_source(&self, value: usize) -> usize {
        if self.source_in_range(value) {
            self.destination + (value - self.source)
        } else {
            value
        }
    }
}

struct FarmingMap {
    ranges: Vec<FarmingRange>,
}

impl FarmingMap {
    fn new() -> Self {
        Self { ranges: vec![] }
    }
}

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
                    seed_vec = seedlist
                        .split_whitespace()
                        .map(|s| s.parse().expect(&format!("Unable to unwrap {s}")))
                        .collect();
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
    let maps: [&FarmingMap; 7] = [
        &seed_to_soil,
        &soil_to_fertilizer,
        &fertilizer_to_water,
        &water_to_light,
        &light_to_temp,
        &temp_to_humidity,
        &humidity_to_location,
    ];
    let mut seed_values: Vec<usize> = vec![];
    for seed in seed_vec {
        let mut seed_value = seed;
        for map in maps {
            for range in &map.ranges {
                if range.source_in_range(seed_value) {
                    seed_value = range.process_source(seed_value);
                    break;
                }
            }
        }
        seed_values.push(seed_value);
    }
    seed_values.sort();
    seed_values[0]
}

#[test]
fn sample_test() {
    assert_eq!(run("sample_input.txt"), 35);
}
