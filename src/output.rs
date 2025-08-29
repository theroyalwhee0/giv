use crate::cli::CommandOptions;

/// Output the result of a command.
/// If the 'json' flag is set, output JSON.
/// Otherwise, output plain text.
///
/// # Arguments
///
/// - `options` The command options.
/// - `value` The output string to be printed.
pub fn outputln(options: CommandOptions, value: impl Into<String>) {
    let value: String = value.into();

    // If 'json' flag is set, output JSON.
    #[cfg(feature = "json")]
    if options.json {
        let json_output = serde_json::to_string(&value).unwrap();
        println!("{}", json_output);
        return;
    }

    // Else output plain text.
    println!("{}", value);
}
