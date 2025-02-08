# Protypo

## 📖 Table of Contents

- [Introduction](#-introduction)
- [Features](#features)
- [Installation](#-installation)
- [Usage](#-usage)
- [Generators](#-generators)
- [Contributing](#-contributing)
- [License](#-license)

## 📝 Introduction

The name Protypo originates from the Greek word πρότυπο (pronounced pro-tý-po), meaning 'model', 'template', or 'pattern'. Protypo is a powerful code generator that transforms entity definitions into fully functional code, allowing you to quickly scaffold entire backend systems, APIs, and more.

Built for speed and flexibility, Protypo brings modern code generation to Rust, combining the performance of a Rust-based tool with the convenience of customizable templates for various programming languages and architectures.

## Features
- 🚀 Supports `MiniJinja` (default) and `Tera` template engines.
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
   cargo install --git https://github.com/dinosath/protypo.git
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