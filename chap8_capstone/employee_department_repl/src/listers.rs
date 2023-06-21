use std::collections::HashMap;

pub fn list_all_departments_unsorted(hm: &HashMap<String, Vec<String>>){
    //TODO: Sort the items.
    let items: Vec<&String> = hm.keys().collect();
    for key in items {
        match hm.get(key) {
            Some(vec) => {
                println!("| {}", key);
                for person in vec {
                    println!("|-- {}", person);
                }
            }
            None => continue,
        };
    }
}

pub fn list_all_departments(hm: &HashMap<String, Vec<String>>){
    //TODO: Sort the items.
    let mut items: Vec<&String> = hm.keys().collect();
    items.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    for key in items {
        match hm.get(key) {
            Some(vec) => {
                println!("| {}", key);
                for person in vec {
                    println!("|-- {}", person);
                }
            }
            None => continue,
        };
    }
}

pub fn list_department(hm: &HashMap<String, Vec<String>>, department: String) {
    match hm.get(&department) {
        Some(vclst) => {
            println!("| {}", department);
            for person in vclst {
                println!("|-- {}", person);
            }
        }
        None => {
            println!("Couldn't find department.");
        }
    }
}