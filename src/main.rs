use std::fs;
use chrono::{Duration, Utc};

fn main() {

    let workout_date = Utc::now().naive_local().date() + Duration::days(2);
    let first_workout = 14;

    let calendar = workout_plans::create_calendar_from_input(workout_date, first_workout);

    fs::write("FitmacherFormel.ics",
              calendar.to_string()).expect("Could not create output");
}
