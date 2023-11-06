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
    complex::Complex,
    Num,
};

/// Prompts the user to enter a complex number.
///
/// # Parameters
/// - prompt: A string slice that will be printed before each part of the complex number is read.
/// 
/// # Returns
/// - `Ok(user_complex_input)` upon success
/// - `Err(io_error)` upon failure
pub fn get_complex(prompt: &str) -> Result<Complex<f64>, std::io::Error> {
    let mut stdout = stdout().lock();
    stdout.write(prompt.as_bytes())?;
    stdout.flush()?;
    
    let real = get_num("Enter the real part: ")?;
    let imaginary = get_num("Enter the imaginary part: ")?;

    let input = Complex::new(real, imaginary);
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

/// Reads a line of input from stdin
fn get_line(prompt: &str) -> Result<String, std::io::Error> {
    // create handles to standard input/output streams.
    let mut stdout = stdout().lock();
    let mut stdin = stdin().lock();
    
    // prompt the user to interact through standard output stream
    stdout.write(prompt.as_bytes())?;
    stdout.flush()?;
    
    // read the user's line of input from the standard input stream.
    let mut input = String::new();
    stdin.read_line(&mut input)?;

    // a slice of input that omitts the whitespace is cloned and the clone is returned
    Ok(input.trim().to_owned())
}