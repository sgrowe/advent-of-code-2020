pub fn start_day(name: &str) -> String {
    println!("Day {}", name);

    read_input_file(name)
}

fn read_input_file(day_number: &str) -> String {
    let file_name = format!("src/inputs/day_{}.txt", day_number);

    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse_ints<'a>(text: &'a str) -> impl Iterator<Item = u64> + 'a {
    text.lines().map(|x| x.parse().unwrap())
}

pub fn get_text_up_to(s: &str, c: char) -> (&str, &str) {
    let end = s.find(c).unwrap();

    (&s[..end], &s[end + 1..])
}
