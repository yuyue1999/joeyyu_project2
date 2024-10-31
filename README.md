

![CI](https://github.com/yuyue1999/joeyyu_project2/actions/workflows/ci.yml/badge.svg)
![CI](https://github.com/yuyue1999/joeyyu_project2/actions/workflows/rust.yml/badge.svg)
# Rust SQLite CRUD CLI Application

This project demonstrates the use of Rust for creating, reading, updating, and deleting (CRUD) records in a SQLite database. The project provides a command-line interface (CLI) application that performs basic user data management, built using Rust with optimizations and continuous integration on GitLab Actions.

## Features

- **Rust Source Code**: Written in Rust, demonstrating full understanding of Rust syntax and features.
- **SQLite Database**: Implements CRUD operations on a SQLite database.
- **Optimized Rust Binary**: Builds an optimized binary using GitLab Actions for enhanced performance.
- **GitLab Actions**: Includes workflow configuration for building, testing, and optimizing Rust code.
- **Video Demo**: [YouTube Demo](https://youtu.be/MuREU6UFNgs) showcasing the functionality of the CLI application.

## Code Summary
### Prerequisites

- **Rust**: Install Rust from [https://rust-lang.org](https://rust-lang.org).
- **SQLite**: Install SQLite for database operations.

### Running the Application

1. **Clone the repository**:
   ```sh
   git clone <repository-url>
   cd rust-sqlite-crud
   ```

2. **Build the project**:
   ```sh
   cargo build --release
   ```

3. **Run the application**:
   ```sh
   cargo run
   ```

4. **Use CLI Commands**: Utilize the CLI commands to create, read, update, or delete user records in the database.

## GitHub Copilot and LLM Usage

GitHub Copilot was used during the development of this project for autocompletion and syntax suggestions. It assisted in defining the CRUD operations by providing helpful completions, especially when working with Rust's database connections and `rusqlite` library. Copilot’s suggestions were particularly beneficial for implementing repetitive CRUD patterns and structuring functions for improved readability.

## GitLab Actions Workflow

The following `.gitlab-rust.yml` configuration file is used to automate building, testing, and uploading the optimized binary:


## Video Demo

For a quick overview of the CLI and its functions, please check out the video demo on [YouTube](https://youtu.be/MuREU6UFNgs).

## License

This project is licensed under the MIT License.
