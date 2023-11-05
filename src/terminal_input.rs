use std::{
    io::{
        stdout,
        stdin,
        Write,
        BufRead,
    },
    str::FromStr,
};
use num::{
    complex::Complex64,
    Num,
};

/// Prompts the user to enter a complex number.
///
/// <br><b>Parameters</b>
/// - prompt: A string slice that will be printed before each part of the complex number is read.
/// 
/// <br><b>Returns</b>
/// - Ok(user_complex_input) upon success
/// - Err(io_error) upon failure
pub fn get_complex(prompt: &str) -> Result<Complex64, std::io::Error> {
    let mut stdout = stdout().lock();
    stdout.write(prompt.as_bytes())?;
    stdout.flush()?;
    
    let real: f64 = get_num("Enter the real part: ")?;
    let imaginary: f64 = get_num("Enter the imaginary part: ")?;

    let input = Complex64::new(real, imaginary);
    return  Ok(input);
}

/// Prompts the user to enter a number.
/// 
/// # Parameters
/// - `prompt`: A string slice that will be printed before user input is read.
/// 
/// # Type Parameters
///  - `T`: A numeric type that can be parsed from a string with a printible error case
/// 
/// # Returns
/// - `Ok(user_num_input)`: When the user inputs a valid number
/// - `Err(io_error)`: When there is an io error from `get_line`
pub fn get_num<T>(prompt: &str) -> Result<T, std::io::Error> 
where
    T: FromStr + Num, // Needs to be parsable number
    T::Err: std::error::Error, // Need to be able to print error if parse fails
{
    // keep trying until the user gets enters a valid number
    loop {
        let input = get_line(prompt)?;
        match input.parse() {
            Ok(number_input) => {
                return Ok(number_input)
            },
            Err(parse_error) => {
                eprintln!("\nInvalid input: {}\n", parse_error)
            },
        }
    }
}

fn get_line(prompt: &str) -> Result<String, std::io::Error> {
    // create handles to standard input/output streams.
    let mut stdout = stdout().lock(); // locking stdout/stdin is faster!
    let mut stdin = stdin().lock();
    let mut input = String::new(); // create a new String to store the user's line of input

    // prompt the user to interact through standard output stream
    stdout.write(prompt.as_bytes())?; // copy the refrenced prompt to the standard output buffer
    stdout.flush()?; // print the contents of the standard output buffer

    // read the user's line of input from the standard input stream.
    stdin.read_line(&mut input)?; // first line in the standard input buffer is moved into the refrenced String.

    // input is assigned to a new String with a copy of a slice of the original that omitts the whitespace
    input = input
        .trim() // returns a slice of the String that doesn't include the leading or trailing whitespace
        .to_owned(); // creates a new String that owns a copy of the slice returned from .trim()

    return Ok(input);
}