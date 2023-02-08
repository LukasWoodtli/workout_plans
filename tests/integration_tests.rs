use chrono::NaiveDate;
use workout_plans::create_calendar_from_input;

#[test]
fn test_output_file() {
    let workout_date = NaiveDate::from_ymd_opt(2020, 5, 29).expect("Wrong start date");
    let first_workout = 1;
    let calendar = create_calendar_from_input(workout_date, first_workout);

    insta::with_settings!({filters => vec![
            (r"DTSTAMP.*", "[creation date]"),
            (r"UID.*", "[UID]"),
        ]}, {

            insta::assert_snapshot!(calendar.to_string());
        });
}