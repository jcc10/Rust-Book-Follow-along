use std::io;
use std::collections::HashMap;

fn main() {
    
    
    println!("Raw array of numbers sepearated by ', '");

    let mut raw_array = String::new();

    io::stdin()
        .read_line(&mut raw_array)
        .expect("Failed to read line");

    let raw_array = raw_array.split(", ");

    let mut num_vec: Vec<i32> = Vec::new();

    for item in raw_array {
        let item_parsed: i32 = match item.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        num_vec.push(item_parsed);
    }

    mean(&num_vec);
    median(&mut num_vec);
    mode(&num_vec);
}

fn mean(num_vec: &Vec<i32>){
    let mut a = 0;
    let mut c = 0;
    for n in num_vec {
        a += n;
        c += 1;
    }
    let a = f64::from(a);
    let c = f64::from(c);
    let t = a / c;
    println!("Mean is {}", t); 
}

fn median(num_vec: &mut Vec<i32>){
    num_vec.sort();
    let middle_raw = num_vec.len() / 2;
    //println!("raw {}, len {}, lenMod {}", middle_raw, num_vec.len(), num_vec.len() % 2);
    if num_vec.len() % 2 == 0 {
        let low = middle_raw;
        let high = middle_raw - 1;
        //println!("a[{}] = {}, a[{}] = {}", low, num_vec[low], high, num_vec[high]);
        let middle = (f64::from(num_vec[low]) + f64::from(num_vec[high])) / 2.0;
        println!("the median is: {}", middle);
    } else {
        let middle_spot = (num_vec.len()) / 2;
        let middle = num_vec[middle_spot];
        println!("the median is: {}", middle);
    }
}

fn mode(num_vec: &Vec<i32>){
    let mut hm: HashMap<&i32, i32> = HashMap::new();
    for item in num_vec {
        let count = hm.entry(item).or_insert(0);
        *count += 1;
    }
    let mut hv = -1;
    let mut hk: Vec<&i32> = Vec::new();
    for (k, v) in hm {
        if v > hv {
            hv = v;
            hk = Vec::new();
            hk.push(k);
        } else if v == hv {
            hk.push(k);
        }
    }
    hk.sort();
    let mut s = String::new();
    for v in hk {
        if s.len() != 0 {
            s += ", ";
        }
        s += &v.to_string();
    }
    println!("the mode is: {}", s);
}