//fn first_two_lines(day: String) -> String {}

use std::str::FromStr;

pub fn print_lyrics() {
    let days_of_christmas = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "evelenth", "twelth",
    ];
    let mut lyrics = String::new();
    for day_id in days_of_christmas {
        lyrics += String::from_str("On the ")
            + day_id.to_string()
            + String::from_str(day_id)
            + String::from_str(
                " day of Christmas\n
                 My true love gave to me\n",
            )
    }
}
