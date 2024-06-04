/// Asks for a line of input from the stdin and returns a string with the exact input recieved without the last newline.  
/// Panics if failed to flush stdout or newline is not found at the end of input.
/// 
/// You can use the `inputln!(<type> {formatting options})` to automatically trim and parse input, and you can use `inputln!(<type>! {formatting options})` to unwrap the parsed value
#[macro_export]
macro_rules! inputln {
    // Line reading:
    () => {
        {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.strip_suffix('\n').unwrap().to_string()
            //(&input[..input.len() - 1]).to_string()
        }
    };

    // Parsing:
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
