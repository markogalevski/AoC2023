use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let result = run("input.txt");
    println!("result = {result}");
}

type SeedRange = std::ops::Range<usize>;

fn add_offset(seed_range: SeedRange, offset: i64) -> SeedRange {
    usize::try_from(seed_range.start as i64 + offset).unwrap()
        ..usize::try_from(seed_range.end as i64 + offset).unwrap()
}

fn convert_seed_range(seed_range: SeedRange, map: &FarmingMap, seed_ranges: &mut Vec<SeedRange>) {
    for range in &map.ranges {
        //case 1 - full fit
        if seed_range.start >= range.start && seed_range.end <= range.end {
            seed_ranges.push(add_offset(seed_range, range.conversion_offset));
            return;
        }
        //case 2.a - partial fit left
        else if seed_range.start < range.start
            && seed_range.end > range.start
            && seed_range.end < range.end
        {
            let external_range = seed_range.start..range.start;
            convert_seed_range(external_range, &map, seed_ranges);
            let internal_range = range.start..seed_range.end;
            seed_ranges.push(add_offset(internal_range, range.conversion_offset));
            return;
        }
        //case 2.b - partial fit right
        else if seed_range.start > range.start
            && seed_range.start < range.end
            && seed_range.end > range.end
        {
            let external_range = range.end..seed_range.end;
            convert_seed_range(external_range, &map, seed_ranges);
            let internal_range = seed_range.start..range.end;
            seed_ranges.push(add_offset(internal_range, range.conversion_offset));
            return;
        }
        //case 2.c - double-ended partial fit
        else if seed_range.start < range.start && seed_range.end > (range.end) {
            let left_external_range = seed_range.start..range.start;
            convert_seed_range(left_external_range, &map, seed_ranges);
            let right_external_range = range.end..seed_range.end;
            convert_seed_range(right_external_range, &map, seed_ranges);
            let internal_range = range.start..range.end;
            seed_ranges.push(add_offset(internal_range, range.conversion_offset));
            return;
        }
        //case 3 - no fit at all
        else if (seed_range.start < range.start && seed_range.end < range.end)
            || (seed_range.start > range.start && seed_range.end > range.end)
        {
            continue;
        }
    }
    seed_ranges.push(seed_range);
    return;
}

#[derive(Debug)]
struct FarmingRange {
    start: usize,
    end: usize,
    conversion_offset: i64,
}

impl FarmingRange {
    fn new(str_description: String) -> Self {
        let split: Vec<&str> = str_description.split_whitespace().collect();
        let destination: i64 = split[0]
            .parse()
            .expect(&format!("Unable to unwrap destination {}", split[0]));
        let start = split[1]
            .parse()
            .expect(&format!("Unable to unwrap start {}", split[1]));
        let length: usize = split[2]
            .parse()
            .expect(&format!("Unable to unwrap length {}", split[2]));
        let end = start + length;
        Self {
            start,
            end,
            conversion_offset: (destination - start as i64),
        }
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
    let mut seed_ranges: Vec<SeedRange> = vec![];
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
                        seed_ranges.push(pair[0]..(pair[0] + pair[1]));
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
            .sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
    }
    let mut input_seed_ranges = seed_ranges.clone();
    let mut output_seed_ranges: Vec<SeedRange> = vec![];
    for map in maps.iter() {
        for seed_range in input_seed_ranges {
            convert_seed_range(seed_range, map, &mut output_seed_ranges);
        }
        input_seed_ranges = output_seed_ranges.clone();
        sort_seed_ranges(&mut input_seed_ranges);
        output_seed_ranges.clear();
    }
    sort_seed_ranges(&mut input_seed_ranges);

    input_seed_ranges[0].start
}

fn sort_seed_ranges(seed_ranges: &mut Vec<SeedRange>) {
    seed_ranges.sort_by(|a, b| {
        let result = a.start.partial_cmp(&b.start).unwrap();
        match result {
            std::cmp::Ordering::Equal => a.end.partial_cmp(&b.end).unwrap(),
            _ => result,
        }
    });
}

#[test]
fn sample_test() {
    assert_eq!(run("sample_input.txt"), 46);
}

#[test]
fn main_test() {
    assert_eq!(run("input.txt"), 15290096);
}
