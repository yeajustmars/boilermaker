# Boilermaker

Boilermaker is a language-agnostic, structured, multi-file, code template system for rapid boilerplate project setup. It's goal is to provide a simple and efficient way to generate project scaffolding, allowing developers to move quickly from common boilerplate setups into more interesting things like features.

> _NOTE_: Boilermaker is currently in beta. For now, only the CLI and website are available. If you have any questions, suggestions, or want to contribute, please open a discussion, an issue or a pull request. For the latter, please read our [Contributing Guidelines](https://github.com/yeajustmars/boilermaker/blob/main/CONTRIBUTING.md).

## Installation

### Rustaceans

```
cargo install boilermaker --version 0.1.0-beta14
```

### Package Managers

#### macOS

```
brew install boilermaker
```

#### Linux

Coming soon! We're working on getting Boilermaker into popular package managers for Linux. If you'd like to help, please see our [Contributing Guidelines](https://github.com/yeajustmars/boilermaker/blob/main/CONTRIBUTING.md) .

#### Binaries

Go to the [releases](https://github.com/yeajustmars/boilermaker/releases) page and download the latest version for your platform. Then, add the downloaded binary to your system's PATH.

> _TIP: Or just use the `curl` install command provided in the specific release in to install to `~/.cargo/bin`. (Note: make sure this directory is no your path.)_

## Usage

Boilermaker is self-documenting. Once installed, you can access the command API, as well as the docs, from the command line.

> You can also access the documentation from the website at [https://boilermaker.dev/docs](https://boilermaker.dev/docs).

### Command API

```
boil help
boil --help

```
### Documentation

```
boil docs list
boil docs view <id-or-name>

```

## Contributing

Boilermaker is actively looking for developers, maintainers and template creators to join our team! It is a young project, and we're excited to get it off the ground and build a strong community around it. If you're interested in contributing, please read our [Contributing Guidelines](https://github.com/yeajustmars/boilermaker/blob/main/CONTRIBUTING.md) to see how to request access and get started!

Another way to contribute, even (or maybe especially) if you're not a coder, is to help us know what templates to build! If you have an idea for a template, please [create a poll](https://github.com/yeajustmars/boilermaker/discussions) in the following format:

- **Discussion Type:** Poll. This allows others to vote on the template idea, and the contributors to focus on the most requested templates first.
- **Template Idea:** [A short, descriptive name for the template]
- **Description:** [A brief description of the template and its purpose]
- **Use Cases:** (optional) [Examples of when this template would be useful]

## License

Boilermaker is licensed under the MIT License.
