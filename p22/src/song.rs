pub fn singer() {
    // Sing on the twelve days of Christmas
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];
    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me:",
            days[i]
        );
        for j in (0..=i).rev() {
            if i == 0 && j == 0 {
                println!("{}.", gifts[j]);
            } else {
                println!("{} {}", if j == 0 { "and" } else { "" }, gifts[j]);
            }
        }
        println!();
    }
}
