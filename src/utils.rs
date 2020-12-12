pub fn filename(day: usize) -> String {
    let data = format!("src/inputs/day{day}.txt", day = day);
    return data;
}

pub fn clean_lines(data: &str) -> Vec<&str> {
    return data
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
}

pub fn chunk_newlines(data: &str) -> Vec<Vec<&str>> {
    // Split vectors of lines delimited by an empty new line
    let mut processed: Vec<Vec<&str>> = Vec::new();
    processed.push(Vec::new());

    for line in data.lines() {
        if line.is_empty() {
            processed.push(Vec::new());
            continue;
        }
        let latest = processed.len() - 1;
        processed[latest].push(line)
    }
    return processed;
}
