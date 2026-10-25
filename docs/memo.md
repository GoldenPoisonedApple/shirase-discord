

文字数制限とかがdiscordにそぐっているかのmd作る

curl -vvvで見る


諦めてファイル名入れてもらう形で
URLとfrom_filenameで
Attachmentsはベクタとしてもつ感じで、add_attachmentで
Imageとその他で分けたいかな

同じfilenameを使用されると面倒なことになるが一応送信できる。これをこのライブラリで責任を持つ

ALT用に添付ファイルには全てattachmentをつけれるようにしたい

avatar_urlとauther.urlはattachment:を使用できない、適当なスキーマだと通らなかった。httpかhttpsのみとする(オリジナル)
embedの中ではis_spoilerは無視される
author.icon_urlとfooter.icon_urlはattachment:を使用可能、スキーマは適当でも通った

API_SPECを用意した方がいいかも


filenameとdataを受け取ってdataはmessageで保管

# 技術メモ
- pub(super): 親モジュール内
- pub(crate): クレート内
- pub(in <module>): 指定したモジュール内

```rust
// newtype: コンポジションの形 要素が一つのタプルを持つ構造体のような感じ
pub struct ImageAttachment(Attachment);

self.0.get_filename() // みたいな感じでアクセスする
```

# 方針
なるべく送信前に分かるエラーは排除して通信を抑える設計

# 将来拡張
ファイルを送信できるようにしたい。今attachmentと書きながらimageと言った方がいいくらいの型名の使い方しているため


# discord仕様(2026/10/24時点)
embed全体で6000文字まで、contentは独立して2000文字まで

MIMEは判定していない。中身の検証も無く、filenameの拡張子のみで判定している。拡張子は無くても送れる。
avatar_urlとauther.urlはattachment:を使用できない、適当なスキーマだと通らなかった。httpかhttpsのみとする(オリジナル)
embedの中ではis_spoilerは無視される

author.icon_urlとfooter.icon_urlはattachment:を使用可能、スキーマは適当でも通った

embedの中ではis_spoilerは無視される

embedでcolorとtimestamp以外が最低1つは必要