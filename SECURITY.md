# Security Policy

## Reporting a Vulnerability

The Apex SDK team takes security vulnerabilities seriously. We appreciate your efforts to responsibly disclose your findings and will make every effort to acknowledge your contributions.

### How to Report

**DO NOT** report security vulnerabilities through public GitHub issues.

Instead, please report them via email to **security@apexsdk.dev**.

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

### What to Include

Please include the following information in your report:

- Type of vulnerability (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the vulnerability
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### Preferred Languages

We prefer all communications to be in English.

## Disclosure Policy

When we receive a security bug report, we will:

1. Confirm the problem and determine the affected versions
2. Audit code to find any similar problems
3. Prepare fixes for all supported releases
4. Release new security fix versions as soon as possible

## Security Updates

Security updates will be released as patch versions and announced via:

- GitHub Security Advisories
- Release notes
- Our Discord server: https://discord.gg/zCDFsBaZJN (#security channel)
- X/Twitter (@apexsdk)

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

## Security Measures

Apex SDK implements several security measures:

### Code Security
- Regular security audits with cargo-audit
- Static analysis with clippy
- Dependency scanning with cargo-deny
- Automated security checks in CI/CD

### Dependencies
- All dependencies are vetted for security issues
- Regular dependency updates via Dependabot
- License compliance checks
- Minimal dependency footprint

### Best Practices
- Compile-time type safety
- No unsafe code without explicit justification
- Comprehensive test coverage
- Memory safety guarantees from Rust

## Contact

- **Primary Security Contact**: security@apexsdk.dev
- **Backup Contact**: partnerships@apexsdk.dev
- **Security Advisories**: https://github.com/apex-sdk/apex-sdk/security/advisories

For more information on securing Apex SDK social media and community channels, see [SOCIAL_MEDIA_SECURITY.md](./SOCIAL_MEDIA_SECURITY.md).

