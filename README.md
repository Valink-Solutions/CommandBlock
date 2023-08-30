# CommandBlock - Rust Library for Minecraft Data Handling

[![Crates.io](https://img.shields.io/crates/v/commandblock.svg)](https://crates.io/crates/commandblock)
[![Documentation](https://docs.rs/commandblock/badge.svg)](https://docs.rs/commandblock/)

> **Warning**
> This library is a work in progress and is not recommended for use in production environments. Currently it is only public for educational and contribution purposes only.

## Purpose

The primary aim of CommandBlock is to provide a versatile Rust-based solution for handling Minecraft data commonly found in formats like NBT, Anvil, and region files. This library is designed to handle data from both Java Edition and Bedrock Edition of Minecraft. The library is being developed to assist projects like [ChunkVault](https://chunkvault.com), where efficient parsing and manipulation of Minecraft world data is essential.

## Features (Planned)

- NBT Data Handling
    - [x] Parse NBT data structures
    - [x] NBT to Serde compatible structures
    - [x] Manipulate NBT data structures
    - [x] Write NBT data structures
    - [] Serialize from struct into NBT data structures
- Anvil Data Handling
    - [ ] Parse Anvil data structures
    - [ ] Interpret Anvil data structures
    - [ ] Manipulate Anvil data structures
    - [ ] Write Anvil data structures
- Region File Support
    - [ ] Read Minecraft region files
    - [ ] Interpret Minecraft region files
    - [ ] Manipulate Minecraft region files
    - [ ] Write Minecraft region files
- Bedrock DB Parsing
    - [ ] Parse Bedrock's LevelDB
    - [ ] Interpret Bedrock's LevelDB data structures
    - [ ] Manipulate Bedrock's LevelDB data structures
    - [ ] Write to Bedrock's LevelDB


## Usage

As of now, the library is under active development and is not ready for production use. You are welcome to explore the code and contribute to its development.

### Installation

To install CommandBlock, add the following to your `Cargo.toml` file:

```toml 
[dependencies]
commandblock = { version = "0.5.0", features = ["serde"] }
```

### Examples

follow the [documentation](https://docs.rs/commandblock) for examples on how to use the library.


## Contributing

Contributions to CommandBlock are highly encouraged! If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix: `git checkout -b feature/your-feature-name`.
3. Commit your changes: `git commit -m "Add your meaningful commit message."`.
4. Push the branch to your fork: `git push origin feature/your-feature-name`.
5. Open a Pull Request on GitHub, explaining your changes and their purpose.

Please note that all contributions are subject to review, and the repository owner maintains the final decision on merging.

## License

This project is under [GNU General Public License v3.0](LICENSE.txt).

## Contact

If you have any questions or suggestions, feel free to [open an issue](https://github.com/Valink-Solutions/CommandBlock/issues) on GitHub.
