use loglet::{Level, log, log_tagged};
use owo_colors::Style;

// Define a global custom Level using Level's constructor
// Use Magenta and Underline to make the prefix stand out dramatically
const WTF: Level = Level::new(
    "WTF", 
    Style::new().magenta().bold().underline(), 
    Style::new().magenta(), 
    true // Route to stderr because it's a critical, confusing error
);

fn main() {
    println!("--- Custom Level Demonstration --- \n");

    let component = "AuthService";
    let unexpected_value = 404;

    println!("-- Normal calls --");

    log(
        WTF,
        format_args!("Received status code {} inside the {}! This shouldn't be physically possible.", unexpected_value, component)
    );

    // Print with a tag
    log_tagged(
        WTF,
        Some(&component),
        format_args!("Received status code {} inside the {}! This shouldn't be physically possible.", unexpected_value, component)
    );

    println!("\n -- Direct calls -- ");

    // Alternatively, you can also invoke print directly
    WTF.print(format_args!("Something went critically wrong!"));

    // with tags as well!
    WTF.print_tagged(Some(&component), format_args!("Something went critically wrong!"));
}
