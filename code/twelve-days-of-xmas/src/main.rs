fn main() {
    let days = [
        (1, "first"),
        (2, "second"),
        (3, "third"),
        (4, "fourth"),
        (5, "fifth"),
        (6, "sixth"),
        (7, "seventh"),
        (8, "eighth"),
        (9, "ninth"),
        (10, "tenth"),
        (11, "eleventh"),
        (12, "twelth"),
    ];
    for (i, s) in days.iter() {
        println!("On the {} day of Christmas", s);
        println!("my true love sent to me:");
        print_presents(*i);
        if *i < 12 {
            println!("");
        }
    }
}

fn print_presents(day: i32) {
    if day == 1 {
        println!("A Partridge in a Pear Tree");
    } else {
        let mut d = day;
        while d > 0 {
            match d {
                12 => println!("{} Drummers Drumming", d),
                11 => println!("{} Pipers Piping", d),
                10 => println!("{} Lords a Leaping", d),
                9 => println!("{} Ladies Dancing", d),
                8 => println!("{} Maids a Milking", d),
                7 => println!("{} Swans a Swimming", d),
                6 => println!("{} Geese a Laying", d),
                5 => println!("{} Golden Rings", d),
                4 => println!("{} Calling Birds", d),
                3 => println!("{} French Hens", d),
                2 => println!("{} Turtle Doves", d),
                1 => println!("and a Partridge in a Pear Tree"),
                _ => (),
            }
            d -= 1;
        }
    }
}
