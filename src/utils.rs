pub fn filename(day: usize) -> String{
    let data = format!("src/inputs/day{day}.txt", day = day);
    return data
}


pub fn clean_lines(data: &str) -> Vec<&str>{
    return data.lines()
      .map(|l| l.trim())
      .filter(|l| !l.is_empty())
      .collect();
}

