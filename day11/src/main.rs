use std::collections::HashMap;

struct PathFinder {
    devices: HashMap<String, Vec<String>>,
}

impl PathFinder {
    fn new(devices: HashMap<String, Vec<String>>) -> PathFinder {
        PathFinder { devices }
    }

    fn search(&self, dest: String) -> u32 {
        if dest == "out" {
            return 1;
        }

        self.devices
            .get(&dest)
            .unwrap()
            .iter()
            .map(|device| self.search(device.to_string()))
            .sum()
    }

    fn cached(&self, dest: &str, specials: &[&str]) -> u64 {
        let mut cache: HashMap<String, u64> = HashMap::new();
        let visited: Vec<String> = Vec::new();
        self.search2(dest, specials, visited, &mut cache)
    }

    fn build_key(dest: &str, visited: &[String]) -> String {
        format!(
            "{}{}",
            dest,
            visited.iter().map(|v| v.to_string()).collect::<String>()
        )
    }

    fn search2(
        &self,
        dest: &str,
        specials: &[&str],
        visited: Vec<String>,
        cache: &mut HashMap<String, u64>,
    ) -> u64 {
        let key = PathFinder::build_key(dest, &visited);
        if let Some(value) = cache.get(&key) {
            return *value;
        }
        if dest == "out" {
            if specials
                .iter()
                .all(|device| visited.contains(&device.to_string()))
            {
                return 1;
            }
            return 0;
        }

        let sum = self
            .devices
            .get(dest)
            .unwrap()
            .iter()
            .map(|device| {
                let mut new_visited = visited.clone();
                if specials.contains(&(device.as_str())) {
                    new_visited.push(device.to_string());
                }
                self.search2(device, specials, new_visited, cache)
            })
            .sum();

        cache.insert(key, sum);
        sum
    }
}

fn main() {
    let file = include_str!("../puzzle.txt");
    let devices: HashMap<String, Vec<String>> = file
        .lines()
        .map(|line| {
            let temp = line.replace(":", "");
            let mut parts = temp.split(" ");
            let key = parts.next().unwrap().to_string();
            let values = parts.map(|v| v.to_string()).collect();
            (key, values)
        })
        .collect();

    let finder = PathFinder::new(devices);
    println!("{}", finder.search("you".to_string()));

    let specials = ["dac", "fft"];
    println!("{}", finder.cached("svr", &specials));
}
