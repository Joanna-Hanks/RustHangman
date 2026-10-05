use std::io;

// Displays the title and introduction for the game.
fn display_welcome() {
    println!("=================================");
    println!("        RUST HANGMAN GAME");
    println!("=================================");
    println!("Welcome to Hangman!");
    println!("Try to guess the hidden word one");
    println!("letter at a time.");
    println!("You have 6 incorrect guesses.");
    println!("=================================");
    println!();
}

// Displays the current Hangman drawing based on the number
// of incorrect guesses the player has made.
fn display_hangman(incorrect_guesses: i32) {
    println!("  +---+");
    println!("  |   |");

    if incorrect_guesses >= 1 {
        println!("  O   |");
    } else {
        println!("      |");
    }

    if incorrect_guesses >= 4 {
        println!(" /|\\  |");
    } else if incorrect_guesses >= 3 {
        println!(" /|   |");
    } else if incorrect_guesses >= 2 {
        println!("  |   |");
    } else {
        println!("      |");
    }

    if incorrect_guesses >= 6 {
        println!(" / \\  |");
    } else if incorrect_guesses >= 5 {
        println!(" /    |");
    } else {
        println!("      |");
    }

    println!("      |");
    println!("=========");
}

// Displays the word with letters that have not been guessed
// replaced by underscores.
fn display_word(word: &str, guessed_letters: &Vec<char>) {
    print!("Word: ");

    for letter in word.chars() {
        if guessed_letters.contains(&letter) {
            print!("{} ", letter);
        } else {
            print!("_ ");
        }
    }

    println!();
}

// Checks whether the player's guessed letter is in the word.
fn check_guess(word: &str, guess: char) -> bool {
    word.contains(guess)
}

// Checks whether the player has already guessed a letter.
fn already_guessed(guessed_letters: &Vec<char>, guess: char) -> bool {
    guessed_letters.contains(&guess)
}

// Checks whether the player has successfully guessed every
// letter in the hidden word.
fn word_is_complete(word: &str, guessed_letters: &Vec<char>) -> bool {
    for letter in word.chars() {
        if !guessed_letters.contains(&letter) {
            return false;
        }
    }

    true
}

// Gets a letter from the player and makes sure the input
// contains exactly one alphabetic character.
fn get_guess() -> char {
    loop {
        println!("Enter a letter:");

        let mut input = String::new();

        // read_line borrows the mutable input variable and
        // places the user's response into it.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let input = input.trim().to_lowercase();

        if input.len() == 1 {
            let guess = input.chars().next().unwrap();

            if guess.is_ascii_alphabetic() {
                return guess;
            }
        }

        println!("Please enter one letter from A-Z.");
        println!();
    }
}

// Asks the player whether they want to play another game.
fn play_again() -> bool {
    loop {
        println!();
        println!("Would you like to play again? (y/n)");

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read input.");

        let answer = answer.trim().to_lowercase();

        if answer == "y" || answer == "yes" {
            return true;
        } else if answer == "n" || answer == "no" {
            return false;
        } else {
            println!("Please enter y or n.");
        }
    }
}

// Runs one complete game of Hangman.
fn play_game(word: &str) {
    // A Vec is used to store every letter the player has guessed.
    // This is also the stretch challenge for this project.
    let mut guessed_letters: Vec<char> = Vec::new();

    // The number of incorrect guesses changes throughout the game,
    // so this variable must be mutable.
    let mut incorrect_guesses = 0;

    // The main game loop continues until the player wins or loses.
    loop {
        println!();
        display_hangman(incorrect_guesses);
        println!();

        display_word(word, &guessed_letters);

        println!("Incorrect guesses: {}", incorrect_guesses);

        // This expression calculates how many incorrect guesses
        // the player has remaining.
        let remaining_guesses = 6 - incorrect_guesses;

        println!("Guesses remaining: {}", remaining_guesses);
        println!();

        // Check whether the player has guessed the entire word.
        if word_is_complete(word, &guessed_letters) {
            println!("Congratulations!");
            println!("You guessed the word: {}!", word);
            break;
        }

        // Check whether the player has reached the maximum
        // number of incorrect guesses.
        if incorrect_guesses >= 6 {
            println!("Game over!");
            println!("The word was: {}.", word);
            break;
        }

        let guess = get_guess();

        // Prevent the player from entering the same letter twice.
        if already_guessed(&guessed_letters, guess) {
            println!("You already guessed '{}'.", guess);
            continue;
        }

        // Store the new guess in the vector.
        guessed_letters.push(guess);

        // Check whether the guessed letter appears in the word.
        if check_guess(word, guess) {
            println!("Good guess! '{}' is in the word.", guess);
        } else {
            println!("Sorry, '{}' is not in the word.", guess);

            // Increase the incorrect guess count by one.
            incorrect_guesses += 1;
        }
    }
}

// The main function is where the program starts.
fn main() {
    // This is an immutable variable because the hidden word
    // does not need to change while the game is running.
    let words = vec![
        "rust",
        "computer",
        "program",
        "keyboard",
        "function",
    ];

    display_welcome();

    // This loop allows the player to play multiple games.
    loop {
        println!("Choose a word number from 1 to 5:");

        // Display the available words as numbered choices.
        // The actual words are hidden from the player.
        for (index, _word) in words.iter().enumerate() {
            println!("{}. {}", index + 1, "_".repeat(words[index].len()));
        }

        println!();

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input.");

        // Parse the player's input from a String into a number.
        let choice: usize = match choice.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a number from 1 to 5.");
                continue;
            }
        };

        // Make sure the selected number is within the valid range.
        if choice < 1 || choice > words.len() {
            println!("Please choose a number from 1 to {}.", words.len());
            continue;
        }

        // Subtract one because vector indexes start at zero.
        let selected_word = words[choice - 1];

        println!();
        println!("Your word has {} letters.", selected_word.len());
        println!("Good luck!");
        println!();

        // Pass a reference to the selected word so play_game()
        // can use it without taking ownership of the value.
        play_game(selected_word);

        // Ask the player if they want another round.
        if !play_again() {
            println!();
            println!("Thanks for playing Rust Hangman!");
            break;
        }

        println!();
    }
}