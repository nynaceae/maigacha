use chrono::{DateTime, Local};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PullType {
    Common,
    Rare,
}
impl FromStr for PullType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "common" => Ok(Self::Common),
            "rare" => Ok(Self::Rare),
            _ => Err("Invalid pull type"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pull {
    pub name: String,
    pub pull_type: PullType,
    pub chance: f64,
}

impl Pull {
    pub fn new(name: String, pull_type: PullType, chance: f64) -> Self {
        Self {
            name,
            pull_type,
            chance,
        }
    }
}

impl FromStr for Pull {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        if parts.len() == 3 {
            let name = parts[0].to_owned();
            let chance = parts[2].parse::<f64>().map_err(|_| "Invalid chance")?;
            let pull_type = PullType::from_str(parts[1])?;

            return Ok(Self {
                name,
                pull_type,
                chance,
            });
        }

        Err("Invalid pull chance string")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullList {
    pub list: Vec<Pull>,
    pub pull_history: PullHistory,
    pub rare_rarity: usize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PullHistory {
    pub history: VecDeque<(DateTime<Local>, PullType, String)>,
    pub size: usize,
}
impl PullHistory {
    pub fn new(size: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(size),
            size,
        }
    }
    pub fn update(&mut self, pull_type: PullType, name: String) {
        let date_time = Local::now();
        self.history.push_back((date_time, pull_type, name));
        if self.history.len() >= self.size {
            self.history.pop_front();
        }
    }
    pub fn contains(&self, pull_type: PullType) -> bool {
        self.history.iter().any(|(_, pt, _)| *pt == pull_type)
    }
    pub fn print(&self) {
        if self.history.is_empty() {
            println!("History is empty.");
        }
        println!(
            "{}",
            self.history
                .iter()
                .map(|(date_time, pull_type, name)| format!(
                    "{} {:#?} \"{}\"",
                    date_time.format("%Y-%m-%d %H:%M:%S"),
                    pull_type,
                    name
                ))
                .collect::<Vec<_>>()
                .join(",\n")
        );
    }
}

impl PullList {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            pull_history: PullHistory::new(35),
            rare_rarity: 100,
        }
    }

    pub fn insert(&mut self, pull: Pull) {
        self.list.push(pull);
    }

    pub fn remove(&mut self, name: &str) -> Option<Pull> {
        if let Some(index) = self.list.iter().position(|pull| pull.name == name) {
            return Some(self.list.remove(index));
        }
        None
    }

    pub fn pull(&mut self) -> Option<&Pull> {
        if self.list.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        let (common, rare): (Vec<&Pull>, Vec<&Pull>) =
            self.list.iter().partition(|pull| match pull.pull_type {
                PullType::Common => true,
                PullType::Rare => false,
            });

        let common_sum: f64 = common.iter().map(|pull| pull.chance).sum();
        let rare_sum: f64 = rare.iter().map(|pull| pull.chance).sum();

        let (pulls, pulls_sum, pulled_type) = if !rare.is_empty()
            && (common.is_empty()
                || rng.gen_range(0..self.rare_rarity) == 0
                || !self.pull_history.contains(PullType::Rare))
        {
            (rare, rare_sum, PullType::Rare)
        } else {
            (common, common_sum, PullType::Common)
        };

        let select = rng.gen_range(0.0_f64..pulls_sum);
        let mut curr_chance = 0.0_f64;

        for pull in (&pulls).iter() {
            curr_chance += pull.chance;
            if curr_chance > select {
                self.pull_history.update(pulled_type, pull.name.clone());
                return Some(*pull);
            }
        }
        unreachable!();
    }

    pub fn save_to_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string(self)?;

        let mut file = File::create(file_path)?;
        file.write_all(json_string.as_bytes())?;

        Ok(())
    }
    pub fn load_from_json_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_contents = read_to_string(file_path)?;

        let pull_list = serde_json::from_str(&file_contents)?;

        Ok(pull_list)
    }

    pub fn print_list(&self) {
        if self.list.is_empty() {
            println!("No items to list");
            return;
        }
        let (common, rare): (Vec<&Pull>, Vec<&Pull>) =
            self.list.iter().partition(|pull| match pull.pull_type {
                PullType::Common => true,
                PullType::Rare => false,
            });
        if !common.is_empty() {
            println!("-Common Pulls-");
            Self::print_pull_vec(&common);
        }
        if !rare.is_empty() {
            println!("-Rare Pulls-");
            Self::print_pull_vec(&rare);
        }
    }
    fn print_pull_vec(pulls: &[&Pull]) {
        let max_length = pulls.iter().map(|pull| pull.name.len()).max().unwrap() + 2;
        for pull in pulls.iter() {
            println!(
                "{:<max_length$} : {}",
                format!("\"{}\"", pull.name),
                pull.chance
            );
        }
    }
}
