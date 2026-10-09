/*
By: <Asen Doiron>
Date: 2026-09-11
Program Details: <Randomly chosen boards where you have yo guess tiles to hit battleships.>
*/
use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::rng();
    let mut randomships: i32 = rng.random_range(1..=10);
    let mut misses = 0;
    let mut hits = 0;

    let mut sunken = vec![];
    let mut totalships = vec!["A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "B5", "C1", "C2", "C3", "C4", "C5", "D1", "D2", "D3", "D4", "D5", "E1", "E2", "E3", "E4", "E5"];

    println!("{}", randomships);

    loop {

        let mut input = String::new();
        println!("Pick a box from 1-25 by typing the letter followed by the number of the corresponding tile and press enter.");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let guess = input.trim().to_uppercase();

        if randomships == 1 && guess == "A1".to_string() || guess == "A2".to_string() || guess == "C3".to_string() || guess == "C4".to_string() || guess == "C5".to_string() && !sunken.contains(&guess) {
            println!("HIT!"); 
                hits += 1;
                sunken.push(guess);
            }
            
        
        else if randomships == 2 && guess == "A3".to_string() || guess == "B3".to_string() || guess == "C3".to_string() || guess == "E2".to_string() || guess == "E3".to_string() && !sunken.contains(&guess) {
            println!("HIT!"); 
                hits += 1;
                sunken.push(guess);
            }

        else if randomships == 3 && guess == "D3".to_string() || guess == "D4".to_string() || guess == "D5".to_string() || guess == "A4".to_string() || guess == "B4".to_string() && !sunken.contains(&guess) {
            println!("HIT!"); 
                hits += 1;
                sunken.push(guess);
            }
           

        else if randomships == 4 && guess == "C5".to_string() || guess == "D5".to_string() || guess == "E5".to_string() || guess == "B1".to_string() || guess == "B2".to_string() && !sunken.contains(&guess) {
           println!("HIT!"); 
                hits += 1;
                sunken.push(guess);
            }
            

        else if randomships == 5 && guess == "D2".to_string() || guess == "D3".to_string() || guess == "D4".to_string() || guess == "E1".to_string() || guess == "E2".to_string() && !sunken.contains(&guess) {
                println!("HIT!"); 
                hits += 1;
                sunken.push(guess);
            }
            

        else if randomships == 6 && guess == "B5".to_string() || guess == "C5".to_string() || guess == "D5".to_string() || guess == "D1".to_string() || guess == "E1".to_string() && !sunken.contains(&guess) {
            println!("HIT!"); 
                hits += 1;
                sunken.push(guess);
            }

        else if randomships == 7 && guess == "A1".to_string() || guess == "A2".to_string() || guess == "A3".to_string() || guess == "A4".to_string() || guess == "A5".to_string() && !sunken.contains(&guess) {
           println!("HIT!"); 
                hits += 1;
                sunken.push(guess);
            }
            
        else if randomships == 8 && guess == "E3".to_string() || guess == "E4".to_string() || guess == "E5".to_string() || guess == "C2".to_string() || guess == "D2".to_string() && !sunken.contains(&guess) {
            println!("HIT!"); 
                hits += 1;
                sunken.push(guess);
            }
            

        else if randomships == 9 && guess == "B1".to_string() || guess == "B2".to_string() || guess == "B3".to_string() || guess == "D4".to_string() || guess == "E4".to_string() && !sunken.contains(&guess) {
           println!("HIT!"); 
                hits += 1;
                sunken.push(guess);
            }
            

        else if randomships == 10 && guess == "D2".to_string() || guess == "D3".to_string() || guess == "D4".to_string() || guess == "A4".to_string() || guess == "B4".to_string() && !sunken.contains(&guess) {
            println!("HIT!"); 
                hits += 1;
                sunken.push(guess);
            }

        else if sunken.contains(&guess) {
            println!("Ship Already Hit");
        }
            
        else if totalships.contains(&guess.as_str()) {
            println!("MISS!");
            misses += 1;
        }

        else {
            println!("Invalid input!");
        }

        if hits == 5 {
            println!("You win!");
            break;
        }
        if misses == 10 {
            println!("You lose!");
            break;
        }
}
}
