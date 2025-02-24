use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_city_group() -> Vec<Vec<String>> {
    let file_path = "district.json";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut city_group = Vec::new();
    let mut group: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        if line.contains(": {") {
            if !group.is_empty() {
                city_group.push(group.clone());
            }
            group.clear();
        }

        if !line.contains(": [") {
            continue;
        }

        let x: &[_] = &[',', ' ', '\n'];
        let line = line.trim_matches(x);
        group.push(line.to_string());
    }

    if !group.is_empty() {
        city_group.push(group.clone());
    }

    city_group
}

fn get_cities(data: &str) -> Vec<String> {
    const X: &[char] = &['[', ']'];
    let mut cities = Vec::new();
    let parts = data.split(": ").collect::<Vec<&str>>();
    cities.push(parts[0].trim().to_string());
    parts[1].trim_matches(X).split(",").for_each(|city| {
        cities.push(city.trim().to_string());
    });

    cities
}

fn calculate_province_count(data: &Vec<String>) -> i32 {
    let mut vec_set: Vec<HashSet<String>> = Vec::new();
    for group in data {
        let cities = get_cities(group);
        let mut flag = false;
        for city in &cities {
            if let Some(set_index) = vec_set.iter().position(|hash_set| hash_set.contains(city)) {
                for city in &cities {
                    vec_set[set_index].insert(city.clone());
                }
                flag = true;
                break;
            }
        }

        if !flag {
        let mut hash_set: HashSet<String> = HashSet::new();

            for city in &cities {
                hash_set.insert(city.clone());
            }
            vec_set.push(hash_set);
        }
    }

    vec_set.len() as i32
}

pub fn count_provinces() -> String {
    let data = get_city_group();
    let mut result = Vec::new();

    for group in data {
        result.push(calculate_province_count(&group));
    }

    result
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(",")
}
