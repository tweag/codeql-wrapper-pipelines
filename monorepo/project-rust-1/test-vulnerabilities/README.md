# Security Test Cases for CodeQL

This directory contains intentionally vulnerable Rust code designed to test CodeQL's security analysis capabilities.

## Vulnerabilities Included

| Vulnerability Type | CWE | File | Function |
|-------------------|-----|------|----------|
| Command Injection | CWE-78 | rust-security-examples.rs | `vulnerable_command_execution` |
| SQL Injection Pattern | CWE-89 | rust-security-examples.rs | `vulnerable_query_builder` |
| Path Traversal | CWE-22 | rust-security-examples.rs | `vulnerable_file_access` |
| Buffer Overflow | CWE-119 | rust-security-examples.rs | `vulnerable_buffer_operation` |
| Insecure TLS | CWE-295 | rust-security-examples.rs | `vulnerable_tls_connection` |

## Usage

1. **Add these files to your Rust project** (project-rust-1)
2. **Run CodeQL analysis** using the provided script:
   ```bash
   ./codeql_cli.sh /path/to/project-rust-1 rust
   ```
3. **Check the SARIF output** for detected security issues

## Expected CodeQL Detections

CodeQL should detect and report:

* Command injection vulnerabilities
* Potential SQL injection patterns
* Path traversal issues
* Unsafe memory operations
* Insecure network configurations

## Important Note

⚠️ **These are intentionally vulnerable code examples for testing purposes only. Never use this code in production!**
