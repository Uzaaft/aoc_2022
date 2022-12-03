use std::{
    fs::File,
    io::{self, Read},
};

#[derive(Debug)]
struct CaloriesList {
    calories: Vec<i32>,
}

impl CaloriesList {
    fn sum(&self) -> i32 {
        self.calories.iter().sum()
    }
}

fn main() {
    let contents = filename_to_string("input.txt").unwrap();
    let calories_list = get_calories(&contents);
    let total_weight_per_elf: Vec<i32> = calories_list.iter().map(|x| x.sum()).collect();
    dbg!(
        "The elf with the highest total weight is: ",
        total_weight_per_elf.iter().max().unwrap()
    );

    // Part two
    let top_three_elves = top_three(&total_weight_per_elf);
    dbg!(
        "The top three elves have a total weight of: ",
        top_three_elves.iter().sum::<i32>()
    );
}


fn top_three(weights: &[i32]) -> Vec<i32> {
    let mut top_three = weights.to_owned();
    top_three.sort();
    top_three.reverse();
    top_three.truncate(3);
    top_three
}

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn get_calories(s: &str) -> Vec<CaloriesList> {
    // Every line contains an int
    // Every group of lines is separated by a blank line
    // Read every group, and create CaloriesList objects
    let mut calories_list = Vec::new();
    let mut calories = Vec::new();
    for line in s.lines() {
        if line.is_empty() {
            calories_list.push(CaloriesList { calories });
            calories = Vec::new();
        } else {
            calories.push(line.parse::<i32>().unwrap());
        }
    }
    calories_list.push(CaloriesList { calories });

    calories_list
}
