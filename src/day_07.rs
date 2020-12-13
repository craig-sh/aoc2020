use crate::utils;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

struct Bag {
    name: String,
    bags: HashMap<String, u32>, //map of bag name to string
}

impl Bag {
    fn is_leaf(&self) -> bool {
        return self.bags.is_empty();
    }
}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn parse_bag(line: &str) -> Bag {
    // light red bags contain 1 bright white bag, 2 muted yellow bags.
    // or
    // dotted black bags contain no other bags.
    let mut contain_split = line.split("contain");

    let name = contain_split
        .next()
        .unwrap()
        .trim()
        .trim_end_matches(" bags");
    let contents = contain_split.next().unwrap();
    let mut bags: HashMap<String, u32> = HashMap::new();
    for content in contents.split(",") {
        match content {
            " no other bags." => (),
            constraint => {
                let mut terms = constraint.trim().split(" ");
                let count: u32 = terms.next().unwrap().parse().unwrap();
                let subbag_name = format!("{} {}", terms.next().unwrap(), terms.next().unwrap());
                bags.insert(subbag_name, count);
            }
        }
    }
    return Bag {
        name: String::from(name),
        bags,
    };
}

fn cached_solver(bag_map: HashMap<String, Bag>) -> HashSet<String> {
    let mut result: HashMap<String, bool> = HashMap::new();

    fn has_gold(
        bag_map: &HashMap<String, Bag>,
        result: &mut HashMap<String, bool>,
        bag_name: &str,
    ) -> bool {
        if result.contains_key(bag_name) {
            return result.get(bag_name).unwrap().clone();
        }
        if bag_map
            .get(bag_name)
            .unwrap()
            .bags
            .contains_key("shiny gold")
        {
            result.insert(bag_name.to_string(), true);
            return true;
        }
        for inner_bag_name in bag_map.get(bag_name).unwrap().bags.keys() {
            if has_gold(bag_map, result, inner_bag_name) {
                result.insert(inner_bag_name.to_string(), true);
                return true;
            }
        }
        return false;
    }

    bag_map
        .keys()
        .filter(|bag_name| has_gold(&bag_map, &mut result, bag_name))
        .map(|x| x.to_string())
        .collect()
}

pub fn solve() {
    let input = read_to_string(utils::filename(7)).unwrap();
    println!("Day 7 part 1: {}", part_1(&input));
    println!("Day 7 part 2: {}", part_2(&input));
}

fn part_1(bag_infos: &str) -> u32 {
    let bag_map: HashMap<String, Bag> = bag_infos
        .lines()
        .map(|line| parse_bag(line))
        .map(|bag| (bag.name.to_string(), bag))
        .collect();
    cached_solver(bag_map).len() as u32
}

fn part_2(bag_infos: &str) -> u32 {
    let bag_map: HashMap<String, Bag> = bag_infos
        .lines()
        .map(|line| parse_bag(line))
        .map(|bag| (bag.name.to_string(), bag))
        .collect();

    let mut count_cache: HashMap<String, u32> = HashMap::new();

    fn total_subbags(
        bag_name: &str,
        bag_map: &HashMap<String, Bag>,
        cache: &mut HashMap<String, u32>,
    ) -> u32 {
        if cache.contains_key(bag_name) {
            return cache.get(bag_name).unwrap().clone();
        }
        if bag_map.get(bag_name).unwrap().is_leaf() {
            cache.insert(bag_name.to_string(), 0);
            return 0;
        }
        let mut total = 0;
        for (inner_bag_name, inner_bag_count) in bag_map.get(bag_name).unwrap().bags.iter() {
            total += inner_bag_count + (inner_bag_count * total_subbags(inner_bag_name, bag_map, cache));
        }
        cache.insert(bag_name.to_string(), total);
        return total;
    }
    total_subbags("shiny gold", &bag_map, &mut count_cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
        assert_eq!(part_2(&input), 32);
    }
}
