use std::error::Error;

static PRIORITY: &str = ".abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let file_content = include_str!("../../../input/day3.txt");
    let rucksack_contents = deserialize_contents(&file_content);
    let compartments: Vec<(&str, &str)> = rucksack_contents
        .iter()
        .map(|contents| get_rucksack_contents(contents))
        .collect();
    let shared_contents: Vec<char> = compartments
        .iter()
        .map(|(first, second)| get_shared_items_2(first, second))
        .collect();
    let priorities: Vec<usize> = shared_contents
        .iter()
        .map(|c| get_shared_item_priority(c))
        .collect();
    // Sum of priorities
    let sum: usize = priorities.iter().sum();
    println!("Sum of priorities: {}", sum);

    // Part 2
    // Do the same, but group every 3rd line
    let grouped_compartments: Vec<Vec<(&str, &str)>> = compartments
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect();

    
}

// Split file contents into lines
fn deserialize_contents(s: &str) -> Vec<&str> {
    s.lines().collect()
}

fn get_rucksack_contents(s: &str) -> (&str, &str) {
    let l = s.len() / 2;
    (&s[0..l], &s[l..l * 2])
}

fn get_shared_items_2(f: &str, l: &str) -> char {
    let mut shared_items = String::new();
    for c in f.chars() {
        if l.contains(c) {
            shared_items.push(c);
        }
    }
    shared_items.chars().collect::<Vec<char>>()[0]
}

fn get_shared_items_3(f: &str, m: &str, l: &str) -> char {
    let mut shared_items = String::new();
    for c in f.chars() {
        if m.contains(c) && l.contains(c) {
            shared_items.push(c);
        }
    }
    shared_items.chars().collect::<Vec<char>>()[0]
}

fn get_shared_item_priority(s: &char) -> usize {
    for (index, c) in PRIORITY.chars().enumerate() {
        if (s == &c) {
            return index;
        }
    }
    unsafe { std::hint::unreachable_unchecked() }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_day3() {
        let example = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let solutions = [
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            ("PmmdzqPrV", "vPwwTWBwg"),
            ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
            ("ttgJtRGJ", "QctTZtZT"),
            ("CrZsJsPPZsGz", "wwsLwLmpwMDw"),
        ];
        let lines = super::deserialize_contents(example);
        for (i, l) in lines.iter().enumerate() {
            let (first_compartment, second_compartment) = super::get_rucksack_contents(l);
            assert_eq!(first_compartment, solutions[i].0);
            assert_eq!(second_compartment, solutions[i].1);
        }

        let shared_solutions = ['p', 'L', 'P', 'v', 't', 's'];
        for (i, l) in lines.iter().enumerate() {
            let (first_compartment, second_compartment) = super::get_rucksack_contents(l);
            let shared_items = super::get_shared_items_2(first_compartment, second_compartment);
            assert_eq!(shared_items, shared_solutions[i]);
        }

        let priority_solutions = [16, 38, 42, 22, 20, 19];
        for (i, l) in lines.iter().enumerate() {
            let (first_compartment, second_compartment) = super::get_rucksack_contents(l);
            let shared_items = super::get_shared_items_2(first_compartment, second_compartment);
            let priority = super::get_shared_item_priority(&shared_items);
            assert_eq!(priority, priority_solutions[i]);
        }
    }
}
