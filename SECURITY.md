# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of Hardware Query seriously. If you believe you've found a security vulnerability, please follow these steps:

1. **Do not disclose the vulnerability publicly**
2. **Email the maintainers** at [ciresnave@gmail.com](mailto:ciresnave@gmail.com) with details about the vulnerability
3. **Include the following information**:
   - Type of vulnerability
   - Full paths of affected source files
   - Location of affected code (line number)
   - Any special configuration required to reproduce
   - Step-by-step instructions to reproduce the issue
   - Proof-of-concept or exploit code (if possible)
   - Impact of the vulnerability

## What to Expect

When you report a vulnerability:

- We will acknowledge receipt of your report within 48 hours
- We will provide a more detailed response within 1 week with our assessment and planned fixes
- We will handle your report with strict confidentiality and not share details with third parties without your permission
- We will keep you informed of our progress
- We will credit you in the security advisory unless you request otherwise

## Security Considerations for Users

The Hardware Query library requires certain system permissions to access hardware information. Be aware of the following security considerations:

1. **Elevated Privileges**: Some hardware detection features may require elevated privileges
2. **Information Disclosure**: This library collects detailed system information which could be sensitive
3. **Dependency Security**: Keep dependencies updated as they may have their own security issues

## Security Development Practices

Our security development practices include:

1. **Dependency Scanning**: Regular scanning of dependencies for known vulnerabilities
2. **Code Reviews**: Security-focused code reviews for all changes
3. **Safe Rust Practices**: Using Safe Rust patterns whenever possible
4. **Minimizing Unsafe Code**: Limiting unsafe blocks to only where necessary and reviewing them thoroughly
