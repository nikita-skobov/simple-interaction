use std::io::prelude::*;
use std::io;

pub enum InteractConfirm {
    YesNo,
    Number(u32),
    Word(String),
}

/// this is the inverse of InteractConfirm. InteractConfirm is used
/// to specify ways a user can confirm an interaction,
/// whereas InteractResult is used as the returned result of
/// what the user selected.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum InteractResult {
    YesNo(bool),
    Number(u32),
    Word(String),
}

pub struct InteractChoices {
    pub confirmation: InteractConfirm,
    pub message: String,
    pub description: Option<String>,
    /// specify the maximum number of times to repeat
    /// the question if the user entered an invalid selection.
    /// if None, then loop forever.
    pub max_loop: Option<usize>,
}

impl InteractChoices {
    pub fn print(&self) -> String {
        let out_str = &self.message;
        match self.confirmation {
            InteractConfirm::YesNo => {
                format!("{} [y/n]:", out_str)
            }
            InteractConfirm::Number(_) => {
                todo!()
            }
            InteractConfirm::Word(_) => {
                todo!()
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
            InteractConfirm::Number(_) => {
                todo!()
            }
            InteractConfirm::Word(_) => {
                todo!()
            }
        };

        Some(result)
    }
}


/// if doing "do you want XYZ?".into()
/// then that should be interpreted as a simple yes/no interact choice
impl<S: AsRef<str>> From<S> for InteractChoices {
    fn from(orig: S) -> Self {
        InteractChoices {
            confirmation: InteractConfirm::YesNo,
            message: orig.as_ref().to_string(),
            description: None,
            max_loop: None,
        }
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
        println!("{}", interact_choices.print());
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
    fn choices_into_works() {
        let mut choice1: InteractChoices = "do you want XYZ?".into();
        choice1.max_loop = Some(1);
        let answer = "y\n";
        let buff = Cursor::new(answer.as_bytes());
        let result = interact_ex(choice1, buff).unwrap();
        assert_eq!(result, InteractResult::YesNo(true));
    }
}
