## Tech Stack

*   **Language**: Rust
*   **Deployment**: bash scripting

## Project Structure and Conventions

Adhere to the following structure and rules when analyzing code:

*   **Error Handling**: Check the overall error handling in the code base and suggest any useful improvements.
*   **Static Analysis & Code Quality**: Check the code base for any areas of improvement with regards to static analysis and code quality.
*   **Testing & Debugging**: Make suggestions on how to improve testing and debugging.
*   **SOLID Principles**: Make useful suggestions on how to ensure the SOLID principles are adhered to.
*   **Handlers**: All database query code must be located in the `src/handlers/` directory. Services should encapsulate business logic and data access.
*   **Cli**: Cli shcema must be located in the `src/cli/` directory. Check the Rust 'struct' for any problematic entries.
*   **Config**: The application must have a config file and relevant Rust 'struct' to ensure minimal command line entries and must be located in the `src/config/` directory.
*   **Scripts**: The application uses this directory to "build and deploy" the service remotely. Check the bash script for any errors, the script  must be located in the `scripts` directory.
