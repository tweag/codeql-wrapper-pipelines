use std::process::Command;
use std::env;

// CWE-78: Command Injection vulnerability
pub fn vulnerable_command_execution(user_input: &str) {
    // This will trigger CodeQL's command injection detection
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("echo {}", user_input)) // Direct user input in command
        .output()
        .expect("Failed to execute command");
    
    println!("Output: {:?}", output);
}

// CWE-89: SQL Injection-like vulnerability (using format strings)
pub fn vulnerable_query_builder(user_id: &str) {
    // This simulates SQL injection patterns that CodeQL detects
    let query = format!("SELECT * FROM users WHERE id = '{}'", user_id);
    println!("Executing query: {}", query);
}

// CWE-22: Path Traversal vulnerability
pub fn vulnerable_file_access(filename: &str) {
    use std::fs;
    
    // This will trigger path traversal detection
    let path = format!("./data/{}", filename); // No sanitization of filename
    match fs::read_to_string(&path) {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}

// CWE-119: Buffer overflow potential
pub fn vulnerable_buffer_operation() {
    let mut buffer = [0u8; 10];
    let user_data = b"This is way too long for the buffer and will cause issues";
    
    // Unsafe copy without bounds checking
    unsafe {
        std::ptr::copy_nonoverlapping(
            user_data.as_ptr(),
            buffer.as_mut_ptr(),
            user_data.len(), // This exceeds buffer size
        );
    }
}

// CWE-295: Improper certificate validation
pub fn vulnerable_tls_connection() {
    // This would typically involve disabling certificate verification
    // CodeQL can detect patterns of insecure TLS configurations
    println!("Connecting with disabled certificate verification...");
}

fn main() {
    // Examples that will trigger CodeQL alerts
    vulnerable_command_execution("$(rm -rf /)"); // Command injection
    vulnerable_query_builder("1' OR '1'='1"); // SQL injection pattern
    vulnerable_file_access("../../../etc/passwd"); // Path traversal
    vulnerable_buffer_operation(); // Buffer overflow
    vulnerable_tls_connection(); // Insecure TLS
}
