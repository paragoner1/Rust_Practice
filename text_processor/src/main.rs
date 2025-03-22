// Build a `text_processor` function. Input is a Vec of (String, Action) tuples.
// Actions: Reverse string, Double it (e.g., "hi" -> "hihi"), Prepend "rust-".
// Output: Vec<String> of processed strings.

enum Action {
    Reverse,
    Double,
    PrependRust,
}

mod processor {
    use super::Action;
    //Here's the function signature
    pub fn text_processor(input: Vec<(String, Action)>) -> Vec<String> {
        // TODO: Implement me!
        let mut output = Vec::new();

        for (mut string, command) in input {
            match command {
                Action::Reverse => string = string.chars().rev().collect(),
                Action::Double => string = string.repeat(2),
                Action::PrependRust => string.insert_str(0, "rust-"),
            }
            output.push(string);
        }

        output // lets go of output (return output)
    }
}

fn main() {//now use the main function to print the output
    let input = vec![
        ("hello".to_string(), Action::Reverse),
        ("world".to_string(), Action::Double),
        ("code".to_string(), Action::PrependRust),
    ];
    let output = processor::text_processor(input);
    println!("{:?}", output);
    // Experiment here.
}

#[cfg(test)]
mod tests {
    use super::processor::text_processor;
    use super::Action;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Action::Reverse),
            ("world".to_string(), Action::Double),
            ("code".to_string(), Action::PrependRust),
        ];
        let output = text_processor(input);
        assert_eq!(output, ["olleh", "worldworld", "rust-code"]);
    }
}
