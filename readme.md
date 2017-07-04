mastodon-bot
---
This is an attempt to build mastodon based [bot](https://botsin.space/@honor) which is written in rust-lang and reads and toots a message to dedicated channel.

Installation
---
To compile and execute this bot below steps can be taken.

1. Create a new `botsin.space` account.
2. Use [Access Token Generator](https://takahashim.github.io/mastodon-access-token/) for Mastodon API
3. Create or Update `.env` file with extracted relevant tokens.
4. build rust-lang application using `cargo build` or `cargo release`.
5. On successful built, Run `cargo run --bin main` to test out logic to retrieve random location.
6. To send toot, Run `./target/debug/toot` or `./target/release/toot`.

Pre-requisite
---
Make sure `cargo` is installed before attempting to build this bot.

Snippets
---
![Alt text](mastodon_bot_shell_01.jpeg?raw=true "Result on shell")

![Alt text](mastodon_bot_site_01.jpeg?raw=true "Result on Mastodon")