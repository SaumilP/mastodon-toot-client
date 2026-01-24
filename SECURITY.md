# Security Policy

## Reporting a Vulnerability

If you discover a vulnerability (e.g., token leakage, unsafe logging, insecure defaults), please report it privately:

- Open a GitHub Security Advisory (preferred), or
- Open an issue and mark it as security-sensitive without pasting secrets.

## Guidance

- Never commit `.env` files
- Never log access tokens
- Prefer GitHub Secrets for scheduled automation
