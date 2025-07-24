# CodeQL Wrapper - Pipeline Examples

A collection of pipeline examples demonstrating how to use the CodeQL Wrapper across different CI/CD platforms.

## Overview

This repository contains practical examples and templates for integrating the **CodeQL Wrapper** into various CI/CD pipelines. The CodeQL Wrapper is a universal Python CLI tool designed to run CodeQL analysis on any type of project (monorepo or single repository) across different CI/CD platforms including Jenkins, GitHub Actions, Harness, Azure Pipelines, and any environment where Python scripts can be executed.

## Supported Platforms

The CodeQL Wrapper has been implemented and tested on the following CI/CD platforms:

| Platform        | Full Scan | Pull Request | Push |
|-----------------|-----------|--------------|------|
| GitHub Actions  | ✅        | ✅           | ✅   |
| Azure Pipelines | ✅        | ✅           | ✅   |
| Circle CI       | ✅        | ✅           | ✅   |
| Harness         | ✅        | ✅           | ✅   |
| Jenkins         | 🔄        | 🚧           | 🚧   |

**Legend:**

* ✅ Fully supported and tested
* 🔄 In development
* 🚧 To be implemented

## Documentation

For complete documentation, installation instructions, and advanced configuration options, visit the main CodeQL Wrapper repository:

**🔗 [CodeQL Wrapper Repository](https://github.com/ModusCreate-Perdigao-GHAS-Playground/codeql-wrapper)**

## Contributing

We welcome contributions! Please see the [contributing guidelines](CONTRIBUTING.md) for more information.

## Support

If you encounter any issues or have questions:

* Check the [main repository documentation](https://github.com/ModusCreate-Perdigao-GHAS-Playground/codeql-wrapper)
* Open an issue in this repository
* Contact the maintainers

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

***

**Note:** This repository contains pipeline examples only. For the actual CodeQL Wrapper tool, please visit the main repository linked above.

***

<div align="center">

**Made with ❤️ by the Modus Create team**

</div>
