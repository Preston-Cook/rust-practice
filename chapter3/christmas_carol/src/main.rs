fn index_to_day(i: i32) -> &'static str {
    return match i {
        0 => "first",
        1 => "second",
        2 => "third",
        3 => "fourth",
        4 => "fifth",
        5 => "sixth",
        6 => "seventh",
        7 => "eighth",
        8 => "ninth",
        9 => "tenth",
        10 => "eleventh",
        11 => "twelfth",
        _ => "invalid",
    };
}

fn main() {
    let lyrics: Vec<&str> = vec![
        "And a partridge in a pear tree.",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 0..12 {
        let mut stanza = format!(
            "On the {} of Christmas,\nMy true love gave to me",
            index_to_day(i)
        );
        if i == 0 {
            stanza = format!("{}A Partridge in a pear tree", stanza);
        } else {
            let mut items = lyrics[0..i as usize + 1].to_vec();
            items.reverse();
            for item in items {
                stanza = format!("{},\n{}", stanza, item);
            }
        }

        println!("{}\n", stanza);
    }
}
