# mastodon-toot-client — Rust CLI bot for scheduled Mastodon posts

A small, production-minded Rust CLI that:
1) Reads a dataset file (CSV/text) or a list of lines,
2) Selects an item (optionally deterministic via `--seed`),
3) Renders a message (optional `--template`), and
4) Posts to a Mastodon instance — or prints output in `--dry-run` mode.

This repo is designed to be **safe to automate** (GitHub Actions / cron), with **quality gates** (fmt/clippy/tests) and **sane defaults**.

---

## Badges

![CI](https://github.com/SaumilP/mastodon-toot-client/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/github/license/SaumilP/mastodon-toot-client)

---

## Features

- **CLI-first** UX (`--dry-run`, `--file`, `--limit`)
- **Safe automation**: never logs tokens, easy secret configuration
- **Deterministic runs** via `--seed` (useful for testing)
- **Template support** for message formatting
- **GitHub Actions** CI + scheduled “Daily Toot” example workflow

---

## Quickstart

### Prerequisites
- Rust (stable) + Cargo
- A Mastodon account on your target instance
- An access token permitted to create statuses

### Configure secrets (local)

Copy `.env.example` to `.env` and fill values:

```bash
cp .env.example .env
```

Run once:

```bash
cargo run -- --dry-run --file ./data/example.txt --limit 1
```

Post for real:

```bash
cargo run -- --file ./data/example.txt --limit 1
```

---

## Configuration

This tool reads config from **(in order)**:
1) CLI flags
2) Environment variables (including `.env`)

### Environment variables

- `MASTODON_BASE_URL` (e.g., `https://botsin.space`)
- `MASTODON_ACCESS_TOKEN` (treat like a password)
- Optional defaults:
  - `INPUT_FILE` (default dataset path)
  - `TOOT_TEMPLATE` (default template)

---

## CLI Usage

```bash
mastodon-toot-client [OPTIONS]

Options:
  -f, --file <PATH>        Input file path (falls back to INPUT_FILE env var)
  -l, --limit <N>          Max number of posts to emit in this run [default: 1]
      --template <STRING>  Message template (falls back to TOOT_TEMPLATE env var)
      --seed <N>           Deterministic seed (useful for tests)
      --dry-run            Print output without posting
      --verbose            Enable more logs
  -h, --help               Print help
```

### Template format

The template supports these placeholders:
- `{item}` — the selected line/record
- `{idx}` — the index selected (0-based)
- `{total}` — total candidate items

Example:

```bash
cargo run -- \
  --dry-run \
  --file ./data/example.txt \
  --template "Today: {item} (#{idx}/{total})"
```

---

## Scheduling (GitHub Actions)

This repo includes a scheduled workflow example: `.github/workflows/schedule-toot.yml`.

### Required secrets

Set these under: **Settings → Secrets and variables → Actions**
- `MASTODON_BASE_URL`
- `MASTODON_ACCESS_TOKEN`

Optional:
- `INPUT_FILE` (if you want to pass a path; otherwise commit a dataset file under `data/`)
- `TOOT_TEMPLATE`

---

## Testing & Quality

Run locally:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

CI runs the same checks on every PR.

---

## Security Notes

- Never commit `.env` or tokens.
- Use GitHub Secrets for automation.
- Tokens must be scoped to the minimum required permissions.

See `SECURITY.md`.

---

## Roadmap

- [ ] CSV schema support (e.g., city name + country fields)
- [ ] Rate limit/backoff and retry strategy
- [ ] Multi-status batching with interval (`--limit N --sleep-ms M`)
- [ ] Optional media uploads
- [ ] Release binaries via GitHub Releases

---

## License

MIT — see `LICENSE`.
