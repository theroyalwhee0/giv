# Security Policy

## Supported Versions

The following versions of `giv` are currently supported with security updates:

| Version | Supported          |
| ------- | ------------------ |
| main    | :white_check_mark: |
| < 1.0   | :x:                |

**Note**: This project is currently in pre-1.0 development. Security fixes are applied to the `main` branch. Once version 1.0 is released, this policy will be updated to reflect supported release versions.

## Reporting a Vulnerability

We take the security of `giv` seriously, especially since the tool generates cryptographic keys and other security-sensitive values.

### How to Report

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report security vulnerabilities via one of the following methods:

1. **GitHub Security Advisories** (preferred): Use GitHub's [private vulnerability reporting](https://github.com/theroyalwhee0/giv/security/advisories/new)
2. **Email**: Contact the maintainer directly (see GitHub profile for contact information)

   You can also send to [security@theroyalwhee.com](mailto:security@theroyalwhee.com). See <https://www.theroyalwhee.com/security/policy/>

### What to Include

Please include as much of the following information as possible:

- Type of vulnerability (e.g., cryptographic weakness, input validation, etc.)
- Step-by-step instructions to reproduce the issue
- Affected versions or commits
- Potential impact of the vulnerability
- Any suggested fixes or mitigations

### What to Expect

- **Initial Response**: Within 72 hours of your report
- **Status Update**: Within 2 weeks
- **Fix Timeline**: Based on severity and complexity
- **Credit**: Security researchers will be credited in release notes unless they prefer to remain anonymous

### Security Scope

Security issues of particular concern for `giv` include:

- Weak or predictable random number generation
- Cryptographic key generation vulnerabilities
- UUID generation that doesn't meet RFC 9562 requirements
- Input validation issues that could lead to code execution
- Dependency vulnerabilities affecting security-critical functionality
- Supply chain security issues

Thank you for helping keep `giv` and its users safe!
