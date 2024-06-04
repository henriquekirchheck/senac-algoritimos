/// Asks for a line of input from the stdin and returns a string with the exact input recieved without the last newline.  
/// Panics if failed to flush stdout or newline is not found at the end of input.
///
/// You can use the `inputln!(<type>)` to automatically trim and parse input, and you can use `inputln!(<type>!)` to unwrap the parsed value
///
/// ```rust,ignore
/// use utils::inputln;
///
/// let input: String = inputln!();
/// let input_with_prompt: String = inputln!("Hello, {}!", "World");
///
/// let input_parsed: Result<f64, _> = inputln!(<f64>);
/// let input_parsed_with_prompt: Result<f64, _> = inputln!(<f64> "Hello, {}!", "World");
///
/// let input_parsed_unwrapped: f64 = inputln!(<f64>!);
/// let input_parsed_with_prompt_unwrapped: f64 = inputln!(<f64>! "Hello, {}!", "World");
/// ```
#[macro_export]
macro_rules! inputln {
    // Line reading:
    () => {
        {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.strip_suffix('\n').unwrap().to_string()
        }
    };

    // Parsing:
    (<$parse:ty>!) => {
        inputln!(<$parse>).unwrap()
    };
    (<$parse:ty>) => {
        {
            let result = inputln!();
            result.trim().parse::<$parse>()
        }
    };
    (<$parse:ty>! $($arg:tt)*) => {
        inputln!(<$parse> $($arg)*).unwrap()
    };
    (<$parse:ty> $($arg:tt)*) => {
        {
            let result = inputln!($($arg)*);
            result.trim().parse::<$parse>()
        }
    };

    // Formatting prompt:
    ($($arg:tt)*) => {
        {
            use std::io::Write;
            print!($($arg)*);
            std::io::stdout().flush().unwrap();
            inputln!()
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_input() {
        let _ = inputln!();
    }

    #[test]
    fn test_input_with_prompt() {
        let _ = inputln!("Hello, {}!", "World");
    }

    #[test]
    fn test_input_parsed() {
        let _ = inputln!(<String>).unwrap();
    }

    #[test]
    fn test_input_parsed_with_prompt() {
        let _ = inputln!(<String> "Hello, {}!", "World").unwrap();
    }

    #[test]
    fn test_input_parsed_unwrapped() {
        let _ = inputln!(<String>!);
    }

    #[test]
    fn test_input_parsed_with_prompt_unwrapped() {
        let _ = inputln!(<String>! "Hello, {}!", "World");
    }
}
