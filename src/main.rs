/*
By: <Asen Doiron>
Date: 2026-09-11
Program Details: <Randomly chosen boards where you have yo guess tiles to hit battleships.>
*/
use rand::Rng;
use std::io::{self, ErrorKind::QuotaExceeded};

fn main() {
    let mut rng = rand::rng();
    let randomships: i32 = rng.random_range(1..=10);
    let mut misses = 0;
    let mut hits = 0;

    let mut sunken = vec![];
    let mut totalships = vec!["A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "B5", "C1", "C2", "C3", "C4", "C5", "D1", "D2", "D3", "D4", "D5", "E1", "E2", "E3", "E4", "E5"];


        println!(" {} |\n {} |\n {} |\n {} |\n {} |", totalships[0], totalships[1], totalships[2], totalships[3], totalships[4]);

    loop {

        let mut input = String::new();
        println!("Pick a box from from a 5 by 5 grid by typing the letter followed by the number of the corresponding tile and press enter.");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let guess = input.trim().to_uppercase();


    if matches!(guess.as_str(), "A1" | "A2" | "C3" | "C4" | "C5")
       && !sunken.contains(&guess)
       {
       println!("HIT!");
       hits += 1;
       sunken.push(guess.clone());
       for i in 0..totalships.len() {
       
    if totalships[i] == guess {
        totalships[i] = "X";
        }
    }
}


    if matches!(guess.as_str(), "A3" | "B3" | "C3" | "E2" | "E3")
       && !sunken.contains(&guess)
       {
       println!("HIT!");
       hits += 1;
       sunken.push(guess.clone());
       for i in 0..totalships.len() {
       
    if totalships[i] == guess {
        totalships[i] = "X";
        }
    }
}

    if matches!(guess.as_str(), "D3" | "D4" | "D5" | "A4" | "B4")
       && !sunken.contains(&guess)
       {
       println!("HIT!");
       hits += 1;
       sunken.push(guess.clone());
       for i in 0..totalships.len() {
       
    if totalships[i] == guess {
        totalships[i] = "X";
        }
    }
}

    if matches!(guess.as_str(), "C5" | "D5" | "E5" | "B1" | "B2")
       && !sunken.contains(&guess)
       {
       println!("HIT!");
       hits += 1;
       sunken.push(guess.clone());
       for i in 0..totalships.len() {
       
    if totalships[i] == guess {
        totalships[i] = "X";
        }
    }
}
    
    if matches!(guess.as_str(), "D2" | "D3" | "D4" | "E1" | "E2")
       && !sunken.contains(&guess)
       {
       println!("HIT!");
       hits += 1;
       sunken.push(guess.clone());
       for i in 0..totalships.len() {
       
    if totalships[i] == guess {
        totalships[i] = "X";
        }
    }
}

    if matches!(guess.as_str(), "B5" | "C5" | "D5" | "D1" | "E1")
       && !sunken.contains(&guess)
       {
       println!("HIT!");
       hits += 1;
       sunken.push(guess.clone());
       for i in 0..totalships.len() {
       
    if totalships[i] == guess {
        totalships[i] = "X";
        }
    }
}

    if matches!(guess.as_str(), "A1" | "A2" | "A3" | "A4" | "A5")
       && !sunken.contains(&guess)
       {
       println!("HIT!");
       hits += 1;
       sunken.push(guess.clone());
       for i in 0..totalships.len() {
       
    if totalships[i] == guess {
        totalships[i] = "X";
        }
    }
}

    if matches!(guess.as_str(), "E3" | "E4" | "E5" | "C2" | "D2")
       && !sunken.contains(&guess)
       {
       println!("HIT!");
       hits += 1;
       sunken.push(guess.clone());
       for i in 0..totalships.len() {
       
    if totalships[i] == guess {
        totalships[i] = "X";
        }
    }
}    
            
    if matches!(guess.as_str(), "B1" | "B2" | "B3" | "D4" | "E4")
       && !sunken.contains(&guess)
       {
       println!("HIT!");
       hits += 1;
       sunken.push(guess.clone());
       for i in 0..totalships.len() {
       
    if totalships[i] == guess {
        totalships[i] = "X";
        }
    }
}
            
    if matches!(guess.as_str(), "D2" | "D3" | "D4" | "A4" | "B4")
       && !sunken.contains(&guess)
       {
       println!("HIT!");
       hits += 1;
       sunken.push(guess.clone());
       for i in 0..totalships.len() {
       
    if totalships[i] == guess {
        totalships[i] = "X";
        }
    }
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
