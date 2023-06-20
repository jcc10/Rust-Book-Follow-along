use std::collections::HashMap;

pub enum AdderMethod{
    GetMut,
    EntryInsert,
}

pub fn adder_wrapper(
    hm: &mut HashMap<String, Vec<String>>,
    user: String,
    department: String,
    method: AdderMethod,
) {
    match method {
        AdderMethod::GetMut => add_get_mut(hm, user, department),
        AdderMethod::EntryInsert => adder_entry_insert(hm, user, department),
    };
}

fn add_get_mut(
    hm: &mut HashMap<String, Vec<String>>,
    user: String,
    department: String,
) {
    match hm.get_mut(&department){
        Some(vec) => {
            vec.push(user);
        },
        None => {
            hm.insert(department, vec![user]);
        }
    };
}


fn adder_entry_insert(
    hm: &mut HashMap<String, Vec<String>>,
    user: String,
    department: String,
) {
    hm.entry(department)
    .or_insert_with(Vec::new)
    .push(user);
}