fn get_input() -> Result<String, Box<std::io::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input)
}

pub fn ask_option_selection(question: String, choices: &[String]) -> usize {
    println!("{}", question);

    for (i, choice) in choices.iter().enumerate() {
        println!("{}. {}", i + 1, choice);
    }

    loop {
        // TODO: improve error handling
        let input = get_input().unwrap();
        let selection = input.trim().parse::<usize>().unwrap();

        if selection > 0 && selection <= choices.len() {
            return selection - 1;
        }

        println!("Invalid input. Please enter a number between 1 and {}.", choices.len());
    }
}

// returns indices of choices made
pub fn ask_multiple_selection(question: String, choices: &[String]) -> Vec<usize> {
    struct Decision(String, bool);

    println!("{}", question);

    let mut decisions: Vec<Decision> = choices
        .iter()
        .map(|choice| Decision(choice.clone(), false))
        .collect();

    loop {
        println!("Select option to toggle, enter nothing to stop");

        for (i, decision) in decisions.iter().enumerate() {
            println!("[{}] {}. {}", if decision.1 { "X" } else { " " }, i + 1, decision.0);
        }

        // TODO: improve error handling
        let input = get_input().unwrap();

        if input.trim().is_empty() {
            break;
        }

        let selection = input.trim().parse::<usize>().unwrap();

        if selection < 1 || selection > choices.len() {
            println!("Selection is invalid");
            continue;
        }

        decisions[selection - 1].1 = !decisions[selection - 1].1;
    }

    decisions
        .iter()
        .enumerate()
        .filter(|(_, decision)| decision.1)
        .map(|(i, _)| i)
        .collect()
}

pub fn ask_optional_positive_number(question: &String) -> Option<u64> {
    println!("{}", question);

    loop {
        // TODO: improve error handling
        let input = get_input().unwrap();

        if input.trim().is_empty() {
            return None;
        }

        let number = input.trim().parse::<u64>();

        match number {
            Ok(n) => {
                return Some(n);
            }
            Err(_) => println!("Invalid input. Please enter a positive number."),
        }
    }
}
