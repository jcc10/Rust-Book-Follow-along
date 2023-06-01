fn main() {
    let text = print_labeled_measurement(5, 'h');
    println!("{}", text);
}

fn print_labeled_measurement(value: i32, unit_label: char) -> String {
    let mut s = String::new();
    s.push_str("The measurement is: ");
    s.push_str(&value.to_string());
    s.push_str(&unit_label.to_string());
    s
}