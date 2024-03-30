use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::{Display, Formatter, Result};

struct Station {
    min: f64,
    sum: f64,
    count: i64,
    max: f64,
}

impl Station {
    fn new() -> Self {
        Self {
            min: f64::MAX,
            sum: 0f64,
            count: 0,
            max: f64::MIN
        }
    }

    fn update(&mut self, temperature: f64) {
        if self.min > temperature {
            self.min = temperature;
        }

        self.sum += temperature;
        self.count += 1;

        if self.max < temperature {
            self.max = temperature;
        }
    }
}

impl Display for Station {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mean = self.sum / self.count as f64;
        write!(f, "{:.1}/{:.1}/{:.1}", self.min, mean, self.max)
    }
}

fn print(map: &HashMap<String, Station>) {
    let mut entries = map.iter().collect::<Vec<_>>();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    let formatted_entries = entries
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join(", ");

    println!("{{{}}}", formatted_entries)
}

fn main() {
    let file = File::open("measurements.txt").expect("Failed to open file");
    let reader = BufReader::with_capacity(4096 * 4096, file);

    let mut map: HashMap<String, Station> = HashMap::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(";").collect();
            if parts.len() != 2 {
                panic!("Incorrect line format: {line}");
            }

            let name = parts[0];
            let temperature = parts[1].parse::<f64>().expect("Invalid temperature format: {parts:?}");

            if let Some(station) = map.get_mut(name) {
                station.update(temperature);
            } else {
                let mut station = Station::new();
                station.update(temperature);
                map.insert(name.to_string(), station);
            }
        }
    }

    print(&map);
}
