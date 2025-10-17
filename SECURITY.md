# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

I take the security of Flux Shell seriously. If you believe you have found a security vulnerability, please follow these steps:

1. **DO NOT** open a public issue
2. Send a description of the vulnerability to [lexi@lrr.sh](mailto:lexi@lrr.sh)
3. Include the following information:
   - Type of vulnerability
   - Full path to source file(s) related to the vulnerability
   - Steps to reproduce
   - Impact of the vulnerability
   - (Optional) Suggested fix

You can expect:

- Acknowledgment of your report within 48 hours
- Regular updates on my progress
- Credit in the security advisory (unless you prefer to remain anonymous)

## Security Considerations

### Plugin System

Flux Shell includes a plugin system that can execute arbitrary code. To minimize risk:

1. Only install plugins from trusted sources
2. Review plugin source code before installation
3. Use the built-in code review feature: `flux plugin install <url>`
4. Plugins run with the same permissions as the shell itself

### Environment Variables

- Internal environment variables are stored encrypted
- Use `env -s KEY=VALUE internal` for sensitive data
- System environment variables are stored in plain text

### Configuration Security

- Config files are stored in user space only
- Permissions are set to `600` (user read/write only)
- Sensitive data should not be stored in the config file

### Best Practices

1. **Plugin Installation**

   ```bash
   # Review code before installing
   flux plugin install <url> # Choose [O]pen for code review
   ```

2. **Sensitive Data**

   ```bash
   # Store sensitive data as internal variables
   flux env -s API_KEY=secret internal
   ```

3. **Configuration**
   ```bash
   # Check file permissions
   ls -l ~/.config/sh.lrr.flux/config.fl
   # Should show: -rw------- (600)
   ```

## Security Features

- Environment variable encryption
- Plugin code review system
- Permission checks on startup
- Secure configuration storage
- Input sanitization
- Path traversal prevention

## Known Issues

Please check my [GitHub Issues](https://github.com/0x800a6/flux/issues) tagged with `security` for any known security issues.

## Security Updates

Security updates will be released as patch versions (0.1.x) and announced through:

1. GitHub Security Advisories
2. Release Notes
3. Our official website

## Auditing

To audit your Flux Shell installation:

1. Check plugin sources:

   ```bash
   flux plugin list
   ```

2. Review environment variables:

   ```bash
   flux env -l internal
   flux env -l system
   ```

3. Verify configuration permissions:
   ```bash
   ls -l ~/.config/sh.lrr.flux/
   ```

## Contributing to Security

We welcome security improvements! To contribute:

1. Fork the repository
2. Create a security enhancement
3. Submit a pull request with detailed description

## Security Team

I can be reached at:

- Email: [lexi@lrr.sh](mailto:lexi@lrr.sh)
- PGP Key: [lrr.sh/pgp-key.txt](https://lrr.sh/pgp-key.txt)

## Acknowledgments

We'd like to thank all security researchers who have helped improve Flux Shell's security. See our [CONTRIBUTORS.md](./.github/CONTRIBUTORS.md) file for details.
