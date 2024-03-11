use clap::Parser;

/// Represents command-line arguments for handling initialization.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Flag indicating whether to perform initialization.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    init: String,
}

/// Parses command-line arguments and returns a boolean value indicating
/// whether initialization should be performed.
///
/// # Examples
///
/// ```
/// use crate::parse_arguments;
///
/// // Simulate command-line arguments
/// let init_flag = parse_arguments();
///
/// // Perform initialization if the flag is true
/// if init_flag {
///     // Perform initialization
///     println!("Initialization command received.");
/// } else {
///     println!("No initialization command received.");
/// }
/// ```
///
/// # Returns
///
/// - `true` if the init command is provided in the command-line arguments.
/// - `false` otherwise.
pub fn parse_arguments() -> bool {
    // Parse command-line arguments into an instance of the Args struct
    let args = Args::parse();
    
    // Attempt to parse the `init` flag as a boolean value
    args.init.parse::<bool>().unwrap_or(false)
}
