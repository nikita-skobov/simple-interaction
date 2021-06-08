# simple-interaction

A very simple library for doing basic terminal interactions (ie: yes/no?, select a number, enter a word).

Use in cargo.toml:

```toml
[dependencies]
simple-interaction = { git = "https://github.com/nikita-skobov/simple-interaction" }
```

Eg:

```rs
use simple_interaction as interact;
use interact::*;

// yes or no:
// Any string implements Into<InteractChoices> and converts it into a yes/no question
let choice1: InteractChoices = "Do you like ice cream?".into();
let choice: bool = interact_yesno(choice1).unwrap();

// numbered choices:
// Any slice of strings implements Into<InteractChoices> and converts it into a numbered choice
let choices = vec!["a", "b", "c"];
let i_choices: InteractChoices = (&choices[..]).into();
let selection: usize = interact_number(i_choices).unwrap();


// word/string input:
// convenience function for making a choices object from a prompt asking
// user to enter a word:
let choices = InteractChoices::choose_word("Please enter your name");
let name: String = interact_word(choices).unwrap();
```

All of the `interact_yesno`, `interact_number`, `interact_word` functions are convenience functions
that take an `InteractChoices` object and returns the relevant type for that object.
Alternatively, you can directly call: `interact(interact_choices)`, but then the result is an
enum of the possible interaction types, and its inconvenient to unwrap. The `interact(..)` function
is called internally on your behalf by the convenience functions, so its suggested to use them.

Theres other options you can set for your `InteractChoices` object before you call the `interact_*` function such
as setting a custom prompt string, setting the maximum times we will loop to ask for user input, adding
a description thats shown to the user, etc..

See the `src/lib.rs` file for the documentation. Its not a big library, fairly readable.

**Remember, this project is very simple, and not very robust. Use at your own risk**

