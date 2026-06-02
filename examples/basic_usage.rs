use loglet::{info, warn, error, debug};

fn main() {
    println!("--- Basic Usage Demonstration --- \n");

    println!(" -- Untagged logging -- ");

    // Info
    info!("Starting the application successfully.");

    // Warn + FormatArgs
    let user: &str = "Alice";
    let attempts: i32 = 3;
    warn!("Failed login attempt by user '{}' (Attempt #{})", user, attempts);

    // Error
    error!("Database connection timed out!");

    // Debug (optimized away in non-debug builds)
    debug!("This is a hidden debug message for developers.");

    println!("\n -- Tagged logging -- ");

    // Tagged ('tag:' is required)
    info!(tag: "auth", "User '{}' logged in successfully.", user);
    warn!(tag: "db", "Connection pool running low ({} remaining).", 2);
    error!(tag: "network", "Failed to reach host after {} retries.", 5);
    debug!(tag: "init", "Loaded config for user '{}'.", user);

    println!("\nLog demonstration complete.");
}

