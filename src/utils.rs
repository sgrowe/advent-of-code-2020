use regex::Captures;

pub fn start_day(name: &str) -> String {
    println!("Day {}", name);

    read_input_file(name)
}

fn read_input_file(day_number: &str) -> String {
    let file_name = format!("src/inputs/day_{}.txt", day_number);

    std::fs::read_to_string(file_name).unwrap()
}

pub fn capture_to_str<'a>(captures: &Captures<'a>, x: usize) -> &'a str {
    captures.get(x).unwrap().as_str()
}

pub fn parse_ints<'a>(text: &'a str) -> impl Iterator<Item = u64> + 'a {
    text.lines().map(|x| x.parse().unwrap())
}
