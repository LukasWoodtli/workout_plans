use base64::engine::general_purpose;
use chrono::{Duration, NaiveDate};
use icalendar::{Calendar, Component, Event, EventLike};
use lazy_static::lazy_static;
use base64::Engine;
use regex::Regex;

struct Workout {
    title: String,
    body: String,
    day: u8
}

impl Workout {
    fn new(title: &str, body: &str) -> Self {
        let day_num: Vec<&str> = title.split(".").collect();
        let day_num = day_num[0];
        let day_num = day_num.parse::<u8>().expect("Not a workout day number");
        Workout {
            title: title.to_string(),
            body: body.to_string(),
            day: day_num }
    }
}

fn get_input() -> String {
    let str = std::fs::read_to_string("input/dfmf.encoded.txt").expect("Can't read input file");
    let str = general_purpose::STANDARD.decode(str).expect("Decode error");
    let str = String::from_utf8(str).expect("Can't decode input");
    return str;
}

fn split_days(text: &str) -> Vec<&str>
{
    lazy_static! {
        static ref SPLIT_DAY_RE: Regex = Regex::new(r"([0-9]{1,2}\. TAG[^\n]*)").expect("Regex error");
    }
    let mut result = Vec::new();

    let mut last = 0;
    for m in SPLIT_DAY_RE.find_iter(text) {

        let index = m.start();

        if last != index {
            result.push(&text[last..index]);
        }
        let matched = m.range();
        result.push(&text[matched.start..matched.end]);
        last = index + &matched.len();

    }
    if last < text.len() {
        result.push(&text[last..]);
    }

    result
}

fn create_workouts(texts: Vec<&str>) -> Vec<Workout> {
    let chunk = texts.chunks(2);
    return chunk.into_iter().map(|c| Workout::new(c[0], c[1])).collect();
}

pub fn create_calendar_from_input(workout_date: NaiveDate, first_workout: u8) -> Calendar {
    let input = get_input();
    let input = split_days(&input);
    let workouts = create_workouts(input);

    let workouts = workouts.into_iter().filter(|w| w.day >= first_workout).collect();

    let calendar = create_workout_calendar(workout_date, workouts);
    calendar
}

fn create_workout_calendar(workout_date: NaiveDate, workouts: Vec<Workout>) -> Calendar {
    let mut calendar = Calendar::new();
    let mut calendar = calendar.name("FitmacherFormel");
    for (i, w) in workouts.into_iter().enumerate() {
        let body = w.title.to_string() + &w.body;
        calendar = calendar.push(
            Event::new()
                .summary(&w.title)
                .description(&body)
                .all_day(workout_date + Duration::days((i) as i64))
                .done(),
        );
    }
    calendar.done()
}
