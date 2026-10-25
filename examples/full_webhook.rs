use bytes::Bytes;
use dotenvy::dotenv;
use std::fs;
use time::OffsetDateTime;
use url::Url;

use shirase::Message;
use shirase::Webhook;
use shirase::error::DiscordError;
use shirase::{Attachment, Author, Color, Embed, Field, Footer, ImageUrl};

fn main() -> Result<(), DiscordError> {
  dotenv().ok();
  let webhook_url = std::env::var("DISCORD_WEBHOOK_URL").unwrap();

  let author_icon = Bytes::from(fs::read("examples/author_icon.png").unwrap());
  let image = Bytes::from(fs::read("examples/image.png").unwrap());
  let thumbnail = image.clone();

  let webhook = Webhook::new(10, 30)?;

  let message = Message::new()
    .content("Hello, world!")?
    .username("username")?
    .avatar_url(Url::parse("https://avatars.githubusercontent.com/u/115403623").unwrap())?
    .tts(true)
    .add_embed(
      Embed::new()
        .title("Hello, world!")?
        .description("This is a test embed.")?
        .author(
          Author::new("John Doe")?
            .url(Url::parse("https://example.com").unwrap())?
            .icon_url(ImageUrl::attachment("author_icon.png")),
        )
        .color(Color::RED)
        .add_field(Field::new("Field 1", "Value 1")?.inline(true))?
        .add_field(Field::new("Field 2", "Value 2")?.inline(true))?
        .add_field(Field::new("Field 3", "Value 3")?.inline(false))?
        .footer(Footer::new("footer")?.icon_url(ImageUrl::url(
          Url::parse("https://avatars.githubusercontent.com/u/115403623").unwrap(),
        )))
        .timestamp(OffsetDateTime::now_utc())
        .image(ImageUrl::attachment("image.png"))
        .thumbnail(ImageUrl::attachment("thumbnail.png")),
    )?
    .add_attachment(Attachment::new("author_icon.png", author_icon)?)
    .add_attachment(Attachment::new("image.png", image)?)
    .add_attachment(Attachment::new("thumbnail.png", thumbnail)?)
    .add_attachment(Attachment::new(
      "file.txt",
      Bytes::from("Hello, world!".as_bytes()),
    )?)
    .build()?;

  let runtime = tokio::runtime::Runtime::new().unwrap();
  runtime.block_on(async {
    webhook.send(&webhook_url, &message).await?;
    Ok(())
  })
}
