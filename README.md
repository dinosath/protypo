# Protypo

## 📖 Table of Contents

- [Introduction](#-introduction)
- [Features](#features)
- [Installation](#-installation)
- [Usage](#-usage)
- [Generators](#-generators)
  - [Generator structure](#generator-structure)
  - [Extending an existing generator](#-extending-an-existing-generator)
- [Contributing](#-contributing)
- [License](#-license)

## 📝 Introduction

Protypo simplifies the process of generating backend code using JSON schemas, allowing you to focus on business logic and reducing repetitive tasks. It can generate code for various languages using customizable templates, making it a versatile tool for API generation, code scaffolding, and more.

Protypo was built to bring the speed and flexibility of low-code tools to Rust, with the extensibility and performance expected from a modern Rust-based tool.

## Features
- 🚀 Supports `Tera` (default) and `MiniJinja` template engines.
- 🛠️ Generates code based on JSON schemas for APIs, microservices, and more.
- 📦 Customizable generators for different frameworks and programming languages.
- 📂 Project structure designed for flexibility and scalability.
- 🔧 Extensible configuration files for defining templates, dependencies, and resources.

## ⚙️ Installation

To install Protypo, follow the steps below:

### Using Cargo

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Install Protypo via Cargo:

   ```bash
   cargo install --git https://github.com/dinosath
   ```

#### 🚀 Usage

Protypo can be used from the command line. Run the following to generate code:

```bash
  protypo generate -p path/to/generator
```

### 📁 Generators

For more info about the generators and how to create a new generator extend an existing one. [documentation](./generators/README.md)

### 🤝 Contributing

Contributions are welcome! If you’d like to contribute to Protypo, feel free to open issues, submit pull requests, or suggest new features.
How to Contribute

1. Fork the repository. 
2. Create a new branch. 
3. Make your changes. 
4. Submit a pull request.

Please ensure your contributions follow the Rust community’s best practices.

### 📄 License

Protypo is licensed under the MIT License. See the LICENSE [here](./LICENSE).