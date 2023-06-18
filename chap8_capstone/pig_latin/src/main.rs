use std::io;

enum Ynu {
    Yes(char),
    No(),
    Unknown(),
}

fn main() {
    
    println!("Write what you need translated to pig latin.");

    let mut raw_vec = String::new();

    io::stdin()
        .read_line(&mut raw_vec)
        .expect("Failed to read line");

    let raw_vec: Vec<&str> = raw_vec.split(" ").collect();
    let piglatin = word_loop(&raw_vec);

    let mut s = String::new();
    for word in piglatin {
        if s.len() != 0 {
            s += " ";
        }
        s += &word;
    }
    println!("{}", s);
}

fn word_loop(word_vec: &Vec<&str>) -> Vec<String>{
    let mut new_word_vec: Vec<String> = Vec::new();
    for word in word_vec {
        new_word_vec.push(letter_loop(&word));
    }
    new_word_vec
}

fn letter_loop(word: &str) -> String {
    let mut new_word = String::new();
    let mut flc: Ynu = Ynu::Unknown();
    let mut added_end: bool = false;
    for letter in word.chars() {
        let drc = char::clone(&letter);
        let drc = drc.to_lowercase().last().unwrap();
        match &flc {
            Ynu::Unknown() => {
                flc = match drc {
                    'a' | 'e' | 'i' | 'o' | 'u' | 'y' => {
                        new_word.push(letter);
                        Ynu::No()
                    }
                    other => {
                        //new_word.push(letter);
                        Ynu::Yes(other)
                    }
                };
            }
            _other => {
                match letter {
                    '.' | ',' | '!' | '?' => {
                        if !added_end{
                            new_word += &gen_end(&flc);
                            added_end = true;
                        }
                        new_word.push(letter);
                    }
                    _other => {
                        new_word.push(letter);
                    }
                }
            }
        }
    }
    if !added_end {
        new_word += &gen_end(&flc);
    }
    new_word
}

fn gen_end(flc: &Ynu) -> String {
    match flc {
        Ynu::Yes(letter) => {
            let mut s = String::new();
            s.push('-');
            s.push(*letter);
            s += "ay";
            s
        },
        Ynu::No() => String::from("-hay"),
        Ynu::Unknown() => {
            println!("How did you get here?");
            String::from("<How?>")
        }
    }
}