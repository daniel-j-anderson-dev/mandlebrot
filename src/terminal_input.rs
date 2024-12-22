use std::{
    io::{stdin, stdout, Write},
    str::FromStr,
};

/// Prompts the user to enter a number.
///
/// # Parameters
/// - `prompt`: A string slice that will be printed before user input is read.
///
/// # Type Parameters
///  - `T`: A type that can be parsed from a string with a printable error case
///
/// # Returns
/// - `Ok(parsed_input)`: When the user inputs a valid instance of `T`
/// - `Err(io_error)`: When there is an io error from `get_line`
pub fn get_parsed_input<T>(prompt: &str) -> Result<T, std::io::Error>
where
    T: FromStr,                // Needs to be parsable
    T::Err: std::error::Error, // Need to be able to print error if parse fails
{
    // keep trying until the user gets enters a valid instance of T
    loop {
        match get_input(prompt)?.parse() {
            Ok(number_input) => return Ok(number_input),
            Err(parse_error) => eprintln!("\nInvalid input: {}\n", parse_error),
        }
    }
}

/// Reads a line of input from stdin
fn get_input(prompt: &str) -> Result<String, std::io::Error> {
    {
        let mut stdout = stdout();
        stdout.write(prompt.as_bytes())?;
        stdout.flush()?;
    }

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    input.truncate(input.trim_end().len());

    Ok(input)
}
