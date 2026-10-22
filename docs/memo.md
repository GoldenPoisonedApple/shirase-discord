文字数制限とかがdiscordにそぐっているか

buildの時cloneしているけど、selfそのまま渡してcloneしない形でもいいいんじゃないか

json!マクロ使った方がいいかも
json!({
    "content": self.content,
})

# 将来拡張
ファイルを送信できるようにしたい。今attachmentと書きながらimageと言った方がいいくらいの型名の使い方しているため