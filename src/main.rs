use std::env;
use std::fs::read_to_string;

/// This function receives a string and escapes the necessary characters.
fn escape_text(input: String) -> String {
    let res = input
        .replace("\n", "\\n")
        .replace("’", "'")
        .replace("\"", "\\\"") // Added this line to escape double quotes
        .replace("`", "'")
        .replace("    ", "\\t");

    res
}

/// This function receives a path to a file, reads its content, and returns it.
fn read_file(path: String) -> std::io::Result<String> {
    read_to_string(path)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a file path as an argument.");
        std::process::exit(1);
    }
    match read_file(args[1].clone()) {
        Ok(input) => {
            let result = escape_text(input);
            println!("{}", result);
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let input = r"# Proposal Title Here
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.

---

Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.

---

## Voting Options

YES: Agree to with this proposal.

NO: Disagree with the proposal.

NO WITH VETO: Disagree with the proposal and want depositors penalized.

ABSTAIN: Decline to give an opinion on the proposal.
";
        let expected = r"# Proposal Title Here\nLorem ipsum dolor sit amet, consectetur adipiscing elit.\nSed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\n---\n\nUt enim ad minim veniam,\nquis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n\n---\n\n## Voting Options\n\nYES: Agree to with this proposal.\n\nNO: Disagree with the proposal.\n\nNO WITH VETO: Disagree with the proposal and want depositors penalized.\n\nABSTAIN: Decline to give an opinion on the proposal.\n";
        assert_eq!(escape_text(input.to_string()), expected.to_string());
    }
}
