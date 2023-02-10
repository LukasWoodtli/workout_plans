use base64::engine::general_purpose;
use chrono::{Duration, NaiveDate};
use icalendar::{Calendar, Component, Event, EventLike};
use base64::Engine;
use crate::dfmf::workout_list::get_workout_list;

mod dfmf;

#[derive(Clone)]
struct Workout {
    title: String,
    body: String,
    day: u8
}

impl Workout {
    fn new(title: String, body: String) -> Self {
        let day_num: Vec<&str> = title.split('.').collect();
        let day_num = day_num[0];
        let day_num = day_num.parse::<u8>().expect("Not a workout day number");
        Workout {
            title,
            body: body.trim_end().to_string(),
            day: day_num }
    }
}

fn deobfuscation(text: &str) -> String {
    let str = general_purpose::STANDARD.decode(text).expect("Decode error");
   String::from_utf8(str).expect("Can't decode input")
}

fn create_workouts() -> Vec<Workout> {
    let workout_list = get_workout_list();
    workout_list.map(|w| Workout::new(deobfuscation(w.title),
                                              deobfuscation(w.body))).to_vec()
}

pub fn create_calendar_from_input(workout_date: NaiveDate, first_workout: u8) -> Calendar {
    let workouts = create_workouts();

    let workouts = workouts.into_iter().filter(|w| w.day >= first_workout).collect();

    create_workout_calendar(workout_date, workouts)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_workout() {
        let workout = Workout::new("1. Tag".to_string(), "abc  \n  ".to_string());
        assert_eq!(workout.day, 1);
        assert_eq!(workout.title, "1. Tag");
        assert_eq!(workout.body, "abc");
    }

    #[test]
    fn test_create_workouts() {
        let workouts = create_workouts();
        assert_eq!(60, workouts.len());
        assert_eq!(workouts[0].day, 1);
        assert!(workouts[0].title.contains("1. TAG"));
        assert!(workouts[0].body.contains("Atmen"));
        assert_eq!(workouts[1].day, 2);
        assert!(workouts[1].title.contains("2. TAG"));
        assert!(workouts[1].body.contains("Muskelkater"));
        assert_eq!(workouts[2].day, 3);
        assert!(workouts[2].title.contains("3. TAG"));
        assert!(workouts[2].body.contains("halte durch"));
        assert_eq!(workouts[59].day, 60);
        assert!(workouts[59].title.contains("60. TAG"));
    }

    #[test]
    fn test_create_workout_calendar() {
        let workouts = create_workouts();
        let cal = create_workout_calendar(NaiveDate::from_ymd_opt(2000, 1, 2).unwrap(),
                                workouts);
        let cal = cal.to_string();
        assert!(cal.contains("X-WR-CALNAME"));
        assert!(cal.contains("DTEND;VALUE=DATE:20000102"));
        assert!(cal.contains("DTSTART;VALUE=DATE:20000102"));
        assert!(cal.contains("DESCRIPTION:3. TAG"));
        assert!(cal.contains("ghi"));
        assert!(cal.contains("SUMMARY:3. TAG"));
    }

    #[test]
    fn test_create_calendar_from_input() {

        let cal = create_calendar_from_input(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
        59);
        let cal = cal.to_string();
        assert!(cal.contains("DESCRIPTION:59. TAG: ENJOY"));
        assert!(cal.contains("DESCRIPTION:60. TAG"));
        assert!(cal.contains("DTEND;VALUE=DATE:20260101"));
        assert!(cal.contains("Pause"));
    }
}
