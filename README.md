# shirase-discord

![Rust](https://img.shields.io/badge/rust-1.99.1%2B-orange.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

**A Rust library for asynchronously  sending Discord webhooks.**

## Features

- Asynchronous sending.
- Send custom messages.
- Performs pre-send validation to minimize unnecessary network requests.
- NO VIBE CODING LIBRARY.

## Installation

```bash
cargo add shirase-discord
```

## Requirements

- **MSRV:** Rust 1.99.1 or later (`rust-version` in `Cargo.toml`)

## Example Usage

- **Async runtime:** `shirase_discord` does not bundle a runtime. `Webhook::send` is `async`; use an executor such as tokio

```toml
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

```rust
use shirase_discord::Webhook;
use shirase_discord::Message;

#[tokio::main]
async fn main() -> Result<(), shirase_discord::error::DiscordError> {
  let webhook = Webhook::new(10, 30)?;
	// message must be built before sending
  let message = Message::new().content("Hello, world!")?.build()?;
  webhook.send("https://discord.com/api/webhooks/...", &message).await?;
  Ok(())
}
```

- For embeds, attachments, and a complete builder walkthrough, see [examples/full_webhook.rs](examples/full_webhook.rs).

## API Specification(at 2026/10/25)

| key             | method                       | restriction                                                                                      | other                                |
| --------------- | ---------------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------------ |
| `content`       | `content(&str)?`             | ..2000 characters                                                                                |                                      |
| `username`      | `username(&str)?`            | 1..80 characters                                                                                 |                                      |
| `avatar_url`    | `avatar_url(Url)?`           | http(s)                                                                                          | shirase_discord original restriction         |
| `tts`           | `tts(bool)`                  |                                                                                                  |                                      |
| `embed`         | `add_embed(Embed)?`          | ..10 embeds, must contain at least text content, `image` or `thumbnail`, total ..6000 characters |                                      |
| ├ `title`       | `title(&str)?`               | ..256 characters                                                                                 |                                      |
| ├ `description` | `description(&str)?`         | ..4096 characters                                                                                |                                      |
| ├ `author`      | `author(Author)`             |                                                                                                  |                                      |
| │└ `name`       | `new(&str)?`                 | ..256 characters                                                                                 |                                      |
| │└ `url`        | `url(Url)?`                  | http(s)                                                                                          | shirase_discord original restriction         |
| │└ `icon_url`   | `icon_url(ImageUrl)`         |                                                                                                  |                                      |
| ├ `color`       | `new(Color(0xXXXXXX))?`      |                                                                                                  | own Color::RED etc.                  |
| ├ `fields`      | `add_field(Field)?`          | ..25 fields                                                                                      |                                      |
| │└ `name`       | `new(&str, &str)?`           | ..256 characters                                                                                 |                                      |
| │└ `value`      | `new(&str, &str)?`           | ..1024 characters                                                                                |                                      |
| │└ `inline`     | `inline(bool)`               |                                                                                                  |                                      |
| ├ `footer`      | `footer(Footer)`             |                                                                                                  |                                      |
| │└ `text`       | `new(&str)?`                 | ..2048 characters                                                                                |                                      |
| │└ `icon_url`   | `icon_url(ImageUrl)`         |                                                                                                  |                                      |
| ├ `timestamp`   | `timestamp(OffsetDateTime)`  |                                                                                                  | ISO 8601                             |
| ├ `image`       | `image(ImageUrl)`            |                                                                                                  |                                      |
| ├ `thumbnail`   | `thumbnail(ImageUrl)`        |                                                                                                  |                                      |
| `attachments`   | `add_attachment(Attachment)` |                                                                                                  |                                      |
| ├ `filename`    | `new(&str, Bytes)?`          | 1..1024 characters, must be unique                                                               | unique(shirase_discord original restriction) |
| ├ `data`        | `new(&str, Bytes)?`          | 1B..20MiB                                                                                        |                                      |
| ├ `description` | `new(&str, Bytes)?`          | 1024 characters                                                                                  |                                      |
| ├ `is_spoiler`  | `spoiler(bool)`              |                                                                                                  | ignored in embed                     |

- Image URL:
  - url(Url): 
  - attachment(&str): An extension must be (*.png, *.jpg, *.jpeg, *.gif, *.webp).
  And must refer to an filename added `add_attachment` method(shirase_discord original restriction).
- A message must contain at least one of the following:
  - `content`
  - `attachment`
  - `embed`
- `components`, `poll` and `allowed_mentions` are not supported(as a one-way flow is assumed)
- `POST` request with:
  - `multipart/form-data` if have attachments
  - `application/json` if not have attachments

## Discord API References

- [Discord Message API](https://discord.com/developers/docs/resources/message)
- [Discord Rate Limits](https://discord.com/developers/docs/topics/rate-limits)
- [Discord API Versioning](https://discord.com/developers/docs/reference)

## Name

- `shirase` means "notification" in Japanese.

## License

This project is licensed under the [MIT License](LICENSE)