https://docs.discord.com/developers/reference  
https://docs.discord.com/developers/topics/rate-limits  
https://docs.discord.com/developers/resources/message

# 技術メモ

- pub(super): 親モジュール内
- pub(crate): クレート内
- pub(in ): 指定したモジュール内

```rust
// newtype: コンポジションの形 要素が一つのタプルを持つ構造体のような感じ
pub struct ImageAttachment(Attachment);

self.0.get_filename() // みたいな感じでアクセスする
```

// cfg: 条件付きコンパイル
#[cfg(feature = "discord")]

application/jsonとmultipart/form-dataを分けてる。

- 通信量を少なくする
- 正しい意味での使い分けとなる

# 方針

なるべく送信前に分かるエラーは排除して通信を抑える設計

ただの一方向通信のため、componentsやpollは実装していない。送信先の事を知らない想定のため、allowed_mentionsも未実装。

# discord仕様(2026/10/24時点)

embed全体で6000文字まで、contentは独立して2000文字まで

MIMEは判定していない。中身の検証も無く、filenameの拡張子のみで判定している。拡張子は無くても送れる。
avatar_urlとauther.urlはattachment:を使用できない、適当なスキーマだと通らなかった。httpかhttpsのみとする(オリジナル)
embedの中ではis_spoilerは無視される

author.icon_urlとfooter.icon_urlはattachment:を使用可能、スキーマは適当でも通った

embedの中ではis_spoilerは無視される

embedでcolorとtimestamp以外が最低1つは必要