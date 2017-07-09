mastodon-toot-client [![Build Status](https://travis-ci.org/SaumilP/mastodon-toot-client.svg)](https://travis-ci.org/SaumilP/mastodon-toot-client)
---
This is an attempt to build mastodon tooter [client](https://botsin.space/@honor) in rust-lang. It reads random location from given file and constructs message with geographical location before tooting to dedicated target mastodon channel.

Installation
---
To compile and execute this client below steps can be taken.

1. Create a new `botsin.space` account.
2. Use [Access Token Generator](https://takahashim.github.io/mastodon-access-token/) for Mastodon API.
3. Create or Update `.env` file with extracted relevant tokens.
4. build rust-lang application using `cargo build` or `cargo release`.
5. On successful built, Run `cargo run --bin main` to test out logic to retrieve random location.
6. Run `cargo test` to execute test cases from `lib.rs`.
7. To send toot, Run `./target/debug/toot` or `./target/release/toot`.

Pre-requisite
---
Make sure `cargo` is installed before attempting to build this client.


Snippets
---
![Alt text](snapshots/mastodon_bot_shell_01.jpeg?raw=true "Result on shell")

![Alt text](snapshots/mastodon_bot_site_01.jpeg?raw=true "Result on Mastodon")
