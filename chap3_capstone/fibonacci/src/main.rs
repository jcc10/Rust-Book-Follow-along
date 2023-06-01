use std::io;

fn main() {
    
    println!("What place in the fibonacci sequence do you want?");

    let mut place_fibonacci = String::new();

    io::stdin()
        .read_line(&mut place_fibonacci)
        .expect("Failed to read line");

    let place_fibonacci: u32 = place_fibonacci.trim().parse().expect("Didn't find a number!");

    let mut n1 = 0;
    let mut n2 = 0;

    for i in 0..(place_fibonacci + 1){
        let current;
        if i == 0 {
            current = 0;
        } else if  i == 1 {
            current = 1;
        } else {
            current = n1 + n2;
        }
        n1 = n2;
        n2 = current;
        //println!("Place {i} is {n2}.");
    }
    println!("Place {place_fibonacci} is {n2}.");
}
