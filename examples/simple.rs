use simple_interaction::*;
use std::io;

pub fn real_main() -> io::Result<()> {
    let yesno_choice: InteractChoices = "Do you like apples?".into();
    let likes_apples = interact_yesno(yesno_choice)?;

    let first_number = if likes_apples { "apples" } else { "bannanas" };
    let dinner_choice = [first_number, "pizza", "ravioli"];
    let mut numbers_choice: InteractChoices = dinner_choice[..].into();
    numbers_choice.description = Some("What would you like to eat?".into());
    let wants_to_eat = interact_number(numbers_choice)?;

    let word_choice = InteractChoices::choose_word("What would you like to drink?");
    let drink = interact_word(word_choice)?;

    println!("You chose to drink {} with your meal of {}", drink, dinner_choice[wants_to_eat - 1]);

    Ok(())
}

pub fn main() {
    if let Err(e) = real_main() {
        eprintln!("Error running examples:\n{}", e);
        std::process::exit(1);
    }
}
