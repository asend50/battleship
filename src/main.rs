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
    let mut waterhit = vec![];
    let possibleships = vec!["A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "B5", "C1", "C2", "C3", "C4", "C5", "D1", "D2", "D3", "D4", "D5", "E1", "E2", "E3", "E4", "E5"];
    let mut totalships = vec!["A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "B5", "C1", "C2", "C3", "C4", "C5", "D1", "D2", "D3", "D4", "D5", "E1", "E2", "E3", "E4", "E5"];
    let mut currentboard = vec![];

    if randomships == 1 {
            currentboard.push("A1");
            currentboard.push("A2");
            currentboard.push("C3");
            currentboard.push("C4");
            currentboard.push("C5");
        }
        else if randomships == 2 {
            currentboard.push("A3");
            currentboard.push("B3");
            currentboard.push("C3");
            currentboard.push("E2");
            currentboard.push("E3");
        }
        else if randomships == 3 {
            currentboard.push("D3");
            currentboard.push("D4");
            currentboard.push("D5");
            currentboard.push("A4");
            currentboard.push("B4");
        }
        else if randomships == 4 {
            currentboard.push("C5");
            currentboard.push("D5");
            currentboard.push("E5");
            currentboard.push("B1");
            currentboard.push("B2");
        }
        else if randomships == 5 {
            currentboard.push("D2");
            currentboard.push("D3");
            currentboard.push("D4");
            currentboard.push("E1");
            currentboard.push("E2");
        }
        else if randomships == 6 {
            currentboard.push("B5");
            currentboard.push("C5");
            currentboard.push("D5");
            currentboard.push("D1");
            currentboard.push("E1");
        }
        else if randomships == 7 {
            currentboard.push("A1");
            currentboard.push("A2");
            currentboard.push("A3");
            currentboard.push("A4");
            currentboard.push("A5");
        }
        else if randomships == 8 {
            currentboard.push("E3");
            currentboard.push("E4");
            currentboard.push("E5");
            currentboard.push("C2");
            currentboard.push("D2");
        }
        else if randomships == 9 {
            currentboard.push("B1");
            currentboard.push("B2");
            currentboard.push("B3");
            currentboard.push("D4");
            currentboard.push("E4");
        }
        else if randomships == 10 {
            currentboard.push("D2");
            currentboard.push("D3");
            currentboard.push("D4");
            currentboard.push("A4");
            currentboard.push("B4");
        }


        println!(" | {} | {} | {} | {} | {} |\n | {} | {} | {} | {} | {} |\n | {} | {} | {} | {} | {} |\n | {} | {} | {} | {} | {} |\n | {} | {} | {} | {} | {} |", totalships[0], totalships[1], totalships[2], totalships[3], totalships[4], totalships[5], totalships[6], totalships[7], totalships[8], totalships[9], totalships[10], totalships[11], totalships[12], totalships[13], totalships[14], totalships[15], totalships[16], totalships[17], totalships[18], totalships[19], totalships[20], totalships[21], totalships[22], totalships[23], totalships[24]);

    loop {

        let mut input = String::new();
        println!("Pick a box from from a 5 by 5 grid by typing the letter followed by the number of the corresponding tile and press enter.");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let guess = input.trim().to_uppercase();


    if sunken.contains(&guess) {
            println!("Ship already hit.");
    }

    if waterhit.contains(&guess) {
            println!("Water already hit.");
        }
    
    else if !possibleships.contains(&guess.as_str()) {
            println!("Invalid input!");
        }
    
    else if randomships == 1 && matches!(guess.as_str(), "A1" | "A2" | "C3" | "C4" | "C5")
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


    else if randomships == 2 && matches!(guess.as_str(), "A3" | "B3" | "C3" | "E2" | "E3")
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

    else if randomships == 3 && matches!(guess.as_str(), "D3" | "D4" | "D5" | "A4" | "B4")
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

    else if randomships == 4 && matches!(guess.as_str(), "C5" | "D5" | "E5" | "B1" | "B2")
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
    
    else if randomships == 5 && matches!(guess.as_str(), "D2" | "D3" | "D4" | "E1" | "E2")
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

    else if randomships == 6 && matches!(guess.as_str(), "B5" | "C5" | "D5" | "D1" | "E1")
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

    else if randomships == 7 && matches!(guess.as_str(), "A1" | "A2" | "A3" | "A4" | "A5")
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

    else if randomships == 8 && matches!(guess.as_str(), "E3" | "E4" | "E5" | "C2" | "D2")
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
            
    else if randomships == 9 && matches!(guess.as_str(), "B1" | "B2" | "B3" | "D4" | "E4")
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
            
    else if randomships == 10 && matches!(guess.as_str(), "D2" | "D3" | "D4" | "A4" | "B4")
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
            
        else if !currentboard.contains(&guess.as_str()) {
            println!("MISS!");
            misses += 1;
            waterhit.push(guess.clone());
        for i in 0..totalships.len() {
            if totalships[i] == guess {
            totalships[i] = "O";
            }
        }
    }

    println!(" | {} | {} | {} | {} | {} |\n | {} | {} | {} | {} | {} |\n | {} | {} | {} | {} | {} |\n | {} | {} | {} | {} | {} |\n | {} | {} | {} | {} | {} |", totalships[0], totalships[1], totalships[2], totalships[3], totalships[4], totalships[5], totalships[6], totalships[7], totalships[8], totalships[9], totalships[10], totalships[11], totalships[12], totalships[13], totalships[14], totalships[15], totalships[16], totalships[17], totalships[18], totalships[19], totalships[20], totalships[21], totalships[22], totalships[23], totalships[24]);

        if hits == 5 {
            println!("You win!");
            break;
        }
        if misses == 15 {
            println!("You lose!");
            break;
        }
}
}
