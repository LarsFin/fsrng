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
