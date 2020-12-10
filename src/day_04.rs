use crate::utils;
use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
enum PassportField{
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
}

fn read_passports(raw_data: &str) -> Vec<HashMap<PassportField, &str>>{
    let mut passports: Vec<HashMap<PassportField, &str>> = Vec::new();
    //
    //Jump start by adding empty element
    passports.push(HashMap::new());

    for line in raw_data.lines(){
        let latest = passports.len() - 1;
        if line.is_empty(){
            if !passports[latest].is_empty(){
                passports.push(HashMap::new());
            }
        } else {
            let vals: Vec<&str> = line.split(" ").collect();
            for val in vals.into_iter() {
                let key_data: Vec<&str> = val.split(":").collect();
                let data = key_data[1];
                let field = match key_data[0] {
                    "byr" => Some(PassportField::BirthYear),
                    "iyr" => Some(PassportField::IssueYear),
                    "eyr" => Some(PassportField::ExpirationYear),
                    "hgt" => Some(PassportField::Height),
                    "hcl" => Some(PassportField::HairColor),
                    "ecl" => Some(PassportField::EyeColor),
                    "pid" => Some(PassportField::PassportID),
                    "cid" => Some(PassportField::CountryID),
                    _ => None
                };
                passports[latest].insert(field.unwrap(), data);
            }
        }
    }
    return passports;
}

pub fn solve() {
    let raw_data = read_to_string(utils::filename(4)).unwrap();
    let passports = read_passports(&raw_data);
    print!("Day 4 part 1: {}\n", part_1(&passports));
    print!("Day 4 part 2: {}\n", part_2(&passports));
}

fn part_1(passports: &Vec<HashMap<PassportField, &str>>) -> usize{
    let mut count = 0;
    let required_fields = vec![
        PassportField::BirthYear,
        PassportField::IssueYear,
        PassportField::ExpirationYear,
        PassportField::Height,
        PassportField::HairColor,
        PassportField::EyeColor,
        PassportField::PassportID,
    ];
    for passport in passports.into_iter(){
        let mut valid = true;
        for field in &required_fields{
            if !passport.contains_key(field){
                valid = false;
                break;
            }
        }
        if valid{
            count += 1;
        }
    }
    return count;
}

fn validate(passport: &HashMap<PassportField, &str>, field: &PassportField) -> bool{
    match field {
        PassportField::BirthYear => {
            let year:usize = passport[field].parse().unwrap();
            year >= 1920 && year <= 2002
        },
        PassportField::IssueYear => {
            let year:usize = passport[field].parse().unwrap();
            year >= 2010 && year <= 2020
        },
        PassportField::ExpirationYear => {
            let year:usize = passport[field].parse().unwrap();
            year >= 2020 && year <= 2030
        },
        PassportField::Height => {
            let to_parse = String::from(passport[&field]);
            let units = &to_parse[to_parse.len() - 2..to_parse.len()];
            let height:usize = match (&to_parse[0..to_parse.len()-2]).parse(){
                Err(_) => return false,
                Ok(f) => f,
            };

            if units == String::from("cm"){
                height >= 150 && height <= 193
            }
            else{
                height >= 59 && height <= 76
            }
        },
        PassportField::HairColor => {
            let hair_color = String::from(passport[field]);
            let mut chars = hair_color.chars();
            let first_char = match chars.next(){
                None => 'x',
                Some(c) => c,
            };
            first_char == '#' && chars.all(char::is_alphanumeric)
        },
        PassportField::EyeColor => {
            match passport[&field] {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false,
            }
        },
        PassportField::PassportID => {
            let id = String::from(passport[&field]);
            id.len() == 9 && id.chars().all(char::is_numeric)
        },
        PassportField::CountryID => true,
    }
}

fn part_2(passports: &Vec<HashMap<PassportField, &str>>) -> usize{
    let mut count = 0;
    let required_fields = vec![
        PassportField::BirthYear,
        PassportField::IssueYear,
        PassportField::ExpirationYear,
        PassportField::Height,
        PassportField::HairColor,
        PassportField::EyeColor,
        PassportField::PassportID,
    ];
    for passport in passports.into_iter(){
        let mut valid = true;
        for field in &required_fields{
            if !passport.contains_key(field){
                valid = false;
                break;
            }
            if !validate(&passport, field){
                valid = false;
                break;
            }
        }
        if valid{
            count += 1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::part_1;
    use super::part_2;
    use super::PassportField;
    use std::collections::HashMap;

    #[test]
    fn test_part1() {
        let mut passports: Vec<HashMap<PassportField, &str>> = vec![HashMap::new()];

        passports[0].insert(PassportField::BirthYear, "test");
        passports[0].insert(PassportField::IssueYear, "test");
        passports[0].insert(PassportField::ExpirationYear, "test");
        passports[0].insert(PassportField::Height, "test");
        passports[0].insert(PassportField::HairColor, "test");
        passports[0].insert(PassportField::EyeColor, "test");
        passports[0].insert(PassportField::PassportID, "test");
        let result = part_1(&passports);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part2() {
        let mut passports: Vec<HashMap<PassportField, &str>> = vec![HashMap::new()];
        passports[0].insert(PassportField::BirthYear, "1980");
        passports[0].insert(PassportField::IssueYear, "2012");
        passports[0].insert(PassportField::ExpirationYear, "2030");
        passports[0].insert(PassportField::Height, "74in");
        passports[0].insert(PassportField::HairColor, "#623a2f");
        passports[0].insert(PassportField::EyeColor, "grn");
        passports[0].insert(PassportField::PassportID, "087499704");
        let result = part_2(&passports);
        assert_eq!(result, 1);
    }
}
