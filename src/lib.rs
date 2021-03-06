use std::io::prelude::*;
use std::io;

pub fn new_err(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}

#[derive(Debug)]
pub enum InteractConfirm {
    YesNo,
    Number,
    Word,
}

/// this is the inverse of InteractConfirm. InteractConfirm is used
/// to specify ways a user can confirm an interaction,
/// whereas InteractResult is used as the returned result of
/// what the user selected.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum InteractResult {
    YesNo(bool),
    Number(usize),
    Word(String),
}

#[derive(Debug)]
pub struct InteractChoice {
    pub message: String,
}

#[derive(Debug)]
pub struct InteractChoices {
    pub confirmation: InteractConfirm,
    /// only use this for yes/no, otherwise use description for longer
    /// interactions, and then the actual choices have their own messages
    pub message: String,
    /// this will be displayed to the user once per interaction loop. it is meant
    /// to serve as a long text field explaining the interaction
    pub description: Option<String>,
    /// specify the maximum number of times to repeat
    /// the question if the user entered an invalid selection.
    /// if None, then loop forever.
    pub max_loop: Option<usize>,
    /// if confirmation type is yes/no, then this vec can/should be empty
    pub choices: Vec<InteractChoice>,

    /// the string that gets output on the same line as the users input.
    /// if None, then the message will be used as the prompt string
    pub prompt_string: Option<String>,
}

impl InteractChoices {
    pub fn choose_word(message: &str) -> InteractChoices {
        InteractChoices {
            confirmation: InteractConfirm::Word,
            message: message.to_string(),
            description: None,
            max_loop: None,
            choices: vec![],
            prompt_string: Some("> ".into()),
        }
    }

    pub fn print(&self) -> String {
        let out_str = &self.message;
        match self.confirmation {
            InteractConfirm::YesNo => {
                format!("{} [y/n]: ", out_str)
            }
            InteractConfirm::Number => {
                // if this vec is empty, what do?
                let mut s = String::from("");
                for (i, msg) in self.choices.iter().enumerate() {
                    s = format!("{}{}. {}\n", s, i + 1, msg.message);
                }
                // remove trailing newline
                s.pop();
                format!("{}{}", out_str, s)
            }
            InteractConfirm::Word => {
                format!("{}:", out_str)
            }
        }
    }

    pub fn get_result(&self, input: &str) -> Option<InteractResult> {
        let result = match self.confirmation {
            InteractConfirm::YesNo => {
                let answer = match input {
                    "y" => true,
                    "Y" => true,
                    "yes" => true,
                    "n" => false,
                    "no" => false,
                    "N" => false,
                    _ => return None,
                };
                InteractResult::YesNo(answer)
            },
            InteractConfirm::Number => {
                let num = input.parse::<usize>().map_or(None, |n| Some(n))?;
                if num > self.choices.len() || num == 0 { // we want selection index to start at 1
                    return None;
                }
                InteractResult::Number(num)
            }
            InteractConfirm::Word => {
                InteractResult::Word(input.to_string())
            }
        };

        Some(result)
    }
}


/// if doing "do you want XYZ?".into()
/// then that should be interpreted as a simple yes/no interact choice
impl From<&str> for InteractChoices {
    fn from(orig: &str) -> Self {
        InteractChoices {
            confirmation: InteractConfirm::YesNo,
            message: orig.to_string(),
            description: None,
            max_loop: None,
            choices: vec![],
            prompt_string: None,
        }
    }
}

impl From<String> for InteractChoices {
    fn from(s: String) -> Self { InteractChoices::from(&s[..]) }
}

impl<S: AsRef<str>> From<&[S]> for InteractChoices {
    fn from(orig: &[S]) -> Self {
        let mut out_vec = vec![];
        for entry in orig {
            out_vec.push(InteractChoice {
                message: entry.as_ref().to_string(),
            });
        }
        InteractChoices {
            confirmation: InteractConfirm::Number,
            message: "".into(),
            description: None,
            max_loop: None,
            choices: out_vec,
            prompt_string: Some("> ".into()),
        }
    }
}

/// convenience function to parse the interact result on users behalf
/// instead of user having to manually match against the result even though
/// they are only looking for a true/false.
/// as such, it is the user's responsibility to ensure that
/// their interact choices object DOES IN FACT have a yes/no confirmation
/// otherwise this will return an error result
pub fn interact_yesno(interact_choices: InteractChoices) -> io::Result<bool> {
    let res = interact(interact_choices)?;
    match res {
        InteractResult::YesNo(b) => Ok(b),
        InteractResult::Number(_) => Err(new_err("Invalid selection type")),
        InteractResult::Word(_) => Err(new_err("Invalid selection type")),
    }
}

/// convenience function to parse the interact result on users behalf
/// instead of user having to manually match against the result even though
/// they are only looking for a number.
/// as such, it is the user's responsibility to ensure that
/// their interact choices object DOES IN FACT have a numbers confirmation
/// otherwise this will return an error result
pub fn interact_number(interact_choices: InteractChoices) -> io::Result<usize> {
    let res = interact(interact_choices)?;
    match res {
        InteractResult::YesNo(_) => Err(new_err("Invalid selection type")),
        InteractResult::Number(n) => Ok(n),
        InteractResult::Word(_) => Err(new_err("Invalid selection type")),
    }
}

pub fn interact_word(interact_choices: InteractChoices) -> io::Result<String> {
    let res = interact(interact_choices)?;
    match res {
        InteractResult::YesNo(_) => Err(new_err("Invalid selection type")),
        InteractResult::Number(_) => Err(new_err("Invalid selection type")),
        InteractResult::Word(w) => Ok(w),
    }
}

/// like the interact() fn, but this
/// takes an abstract readable interface instead of
/// defaulting to using stdin(). this is mostly used for testing
pub fn interact_ex<R: BufRead>(
    interact_choices: InteractChoices,
    input: R,
) -> io::Result<InteractResult> {
    let mut input = input;
    // the description might be long, so this only gets printed once
    if let Some(ref description) = interact_choices.description {
        println!("{}", description);
    }

    let mut result = None;
    let mut attempts = 0;
    while result.is_none() {
        print!("{}", interact_choices.print());
        if let Some(ref prompt) = interact_choices.prompt_string {
            print!("\n{}", prompt);
        }
        let _ = io::stdout().flush();
        let mut s = String::from("");
        input.read_line(&mut s)?;
        let s_trimmed = s.trim_end();
        result = interact_choices.get_result(s_trimmed);
        if result.is_none() {
            println!("You entered '{}' which doesn't seem to be a valid selection. Please try again, or exit", s_trimmed);
        }

        attempts += 1;
        if let Some(max) = interact_choices.max_loop {
            if attempts >= max {
                break;
            }
        }
    }

    let result = if let Some(r) = result { r } else {
        // if we reached max attempts:
        let err = io::Error::new(io::ErrorKind::Other, "Failed to get an appropriate selection from the user after several attempts");
        return Err(err);
    };

    Ok(result)
}


pub fn interact(interact_choices: InteractChoices) -> io::Result<InteractResult> {
    let stdin_handle = io::stdin();
    interact_ex(interact_choices, stdin_handle.lock())
}


#[cfg(test)]
mod tests {
    use super::*;
    use io::Cursor;

    #[test]
    fn yesno_choices_into_works() {
        let mut choice1: InteractChoices = "do you want XYZ?".into();
        choice1.max_loop = Some(1);
        let answer = "y\n";
        let buff = Cursor::new(answer.as_bytes());
        let result = interact_ex(choice1, buff).unwrap();
        assert_eq!(result, InteractResult::YesNo(true));
    }

    #[test]
    fn list_choices_into_works() {
        let mut choice1 = InteractChoices::from(
            &["apples", "oranges", "babnannamas"][..]
        );
        choice1.max_loop = Some(1);
        let answer = "2\n";
        let buff = Cursor::new(answer.as_bytes());
        let result = interact_ex(choice1, buff).unwrap();
        assert_eq!(result, InteractResult::Number(2));
    }
}
