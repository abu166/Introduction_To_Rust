# Rust Crates Lab: Introduction to Crates, Modules, and Packaging

This project is part of a lab assignment that explores fundamental concepts of Rust programming, including **crates**, **modules**, and **packaging**. These are core building blocks for organizing, managing, and distributing Rust code effectively.

## How Crates and Modules Were Used in This Project

* **Crates:** A crate is the smallest unit of compilation in Rust. In this project:
  * We used the external crate `rand` to generate random numbers. This demonstrates how crates allow us to leverage pre-existing libraries, reducing the need to write repetitive code and improving productivity.
  * External crates are added to the `Cargo.toml` file under `[dependencies]`.

* **Modules:** Modules help organize code into reusable and maintainable components. In this project:
  * We created a custom module named `math_operations` that contains two functions: `add(a, b)` and `multiply(a, b)`.
  * The `main.rs` file imports and uses this module, showcasing how modules promote separation of concerns and modularity.

Overall, crates and modules are integral to Rust's design philosophy, promoting code reuse, organization, and scalability.

## Importance of Crates and Modules

* **Crates:** Crates facilitate code reuse by allowing developers to share and use pre-built libraries. They also make it easy to manage dependencies and ensure compatibility across projects.
* **Modules:** Modules help structure larger projects by grouping related functionality. This improves readability, maintainability, and testability of the codebase.

## Project Structure

The project has the following structure:

```
rust_crates_lab/
├── Cargo.toml    # Configuration file with metadata and dependencies
├── Cargo.lock    # Lock file for dependency versions
├── src/
│   ├── main.rs   # Main entry point of the program
│   └── math_operations.rs # Custom module with reusable functions
└── target/       # Compiled output and packaged files
```

## Step-by-Step Instructions to Run the Code

### Step 1: Install Rust

If you haven't installed Rust yet, follow these steps:

1. Visit the official Rust website: https://www.rust-lang.org/tools/install
2. Run the installation script:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Verify the installation:

```bash
rustc --version
cargo --version
```

### Step 2: Clone the Project

If you have the project as a zip file, extract it to your desired location. Alternatively, clone the repository (if applicable):

```bash
git clone <repository-url>
cd rust_crates_lab
```

### Step 3: Build the Project

Use the following command to build the project and ensure there are no errors:

```bash
cargo build
```

### Step 4: Run the Project

Run the project using:

```bash
cargo run
```

You should see output similar to:
```
Random Number: <some_random_number>
Sum: 12
Product: 48
```

### Step 5: Package the Project

To create a distributable package of the project:

1. Ensure all changes are committed to Git (optional but recommended):

```bash
git init
git add .
git commit -m "Initial commit"
```

2. Run the packaging command:

```bash
cargo package
```

The packaged `.crate` file will be located in the `target/package` directory:
```
target/package/rust_crates_lab-0.1.0.crate
```

## Dependencies

The project uses the following external crate:
* `rand`: For generating random numbers. Added to `Cargo.toml` under `[dependencies]`.

## Contributing

If you'd like to contribute to this project:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/YourFeatureName`).
3. Commit your changes (`git commit -m "Add some feature"`).
4. Push to the branch (`git push origin feature/YourFeatureName`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Acknowledgments

* Thanks to the Rust community for providing excellent documentation and tools.
* Special thanks to the creators of the `rand` crate for their useful library.