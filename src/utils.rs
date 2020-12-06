use regex::Captures;

pub fn read_input_file(day_number: &str) -> String {
    std::fs::read_to_string(format!("src/inputs/day_{}.txt", day_number)).unwrap()
}

pub fn capture_to_str<'a>(captures: &Captures<'a>, x: usize) -> &'a str {
    captures.get(x).unwrap().as_str()
}

pub fn parse_ints(text: &str) -> Vec<u64> {
    text.lines().map(|x| x.parse().unwrap()).collect()
}
