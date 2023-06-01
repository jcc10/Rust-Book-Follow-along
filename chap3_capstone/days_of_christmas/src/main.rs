use std::convert::TryFrom;

const EVENTS: [fn(i32); 12] = [one, two, three, four, five, six, seven, eight, nine, ten, eleven, twelve];

const DAY_NAMES: [&str; 12] = ["First", "Second", "Third", "Fourth", "Fith", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth", "Eleventh", "Twelveth"];

fn main() {
    println!("");
    for day in 1..=12 {
        zero(day);
        for event in (0..day).rev() {
            let event_number = total_gen(day, event);
            let event = usize::try_from(event).unwrap();
            EVENTS[event](event_number)
        }
        println!("")
    }
    
}

fn total_gen(mul: i32, base: i32) -> i32 {
    let base = base + 1;
    let mul = mul - base;
    let mul = mul + 1;
    base * mul
}

fn zero(day: i32) {
    let day = usize::try_from(day).unwrap();
    let day_name: &str = DAY_NAMES[day - 1];
    println!("On the {day_name} day of Christmas my true love sent to me...")
}

fn one(total: i32){
    if total == 1 {
        println!("A partridge in a pear tree. [{total}]");
    } else {
        println!("And a partridge in a pear tree. [{total}]")
    }
}

fn two(total: i32){
    println!("Two turtle doves, [{total}]");
}

fn three(total: i32){
    println!("Three French hens,, [{total}]");
}

fn four(total: i32){
    println!("four calling birds, [{total}]");
}

fn five(total: i32){
    println!("five gold rings, [{total}]");
}

fn six(total: i32){
    println!("six geese a-laying, [{total}]");
}

fn seven(total: i32){
    println!("seven swans a-swimming, [{total}]");
}

fn eight(total: i32){
    println!("eight maids a-milking, [{total}]");
}

fn nine(total: i32){
    println!("nine ladies dancing, [{total}]");
}

fn ten(total: i32){
    println!("ten lords a-leaping, [{total}]");
}

fn eleven(total: i32){
    println!("eleven pipers piping, [{total}]");
}

fn twelve(total: i32){
    println!("twelve drummers drumming, [{total}]");
}