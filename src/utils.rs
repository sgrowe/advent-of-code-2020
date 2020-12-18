use std::fmt::Debug;
use std::str::FromStr;

pub fn start_day(name: &str) -> String {
    println!("Day {}", name);

    read_input_file(name)
}

pub fn read_input_file(day_number: &str) -> String {
    let file_name = format!("src/inputs/day_{}.txt", day_number);

    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse_ints<'a, I, Err>(text: &'a str) -> impl Iterator<Item = I> + 'a
where
    I: FromStr<Err = Err>,
    Err: Debug,
{
    text.lines().map(|x| x.parse().unwrap())
}

pub fn get_text_up_to(s: &str, c: char) -> (&str, &str) {
    let end = s.find(c).unwrap();

    (&s[..end], &s[end + 1..])
}

pub fn first<I, X>(collection: I) -> X
where
    I: IntoIterator<Item = X>,
{
    collection.into_iter().next().unwrap()
}
