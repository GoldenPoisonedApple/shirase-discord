# Discord Incoming Webhook 送信仕様

本ドキュメントは、**アプリケーションから Discord へメッセージを送る**ケースのみを整理したものである。Webhook の作成・削除や、Discord からアプリへ届く Webhook Events は対象外。

公式リファレンス:

- [Webhook Resource](https://discord.com/developers/docs/resources/webhook)
- [Allowed Mentions](https://discord.com/developers/docs/resources/message#allowed-mentions-object)
- [Embed Object](https://discord.com/developers/docs/resources/message#embed-object)
- [Uploading Files](https://discord.com/developers/docs/reference#uploading-files)
- [Poll Request Object](https://discord.com/developers/docs/resources/poll#poll-create-request-object)
- [Rate Limits](https://discord.com/developers/docs/topics/rate-limits)

---

## 共通

### エンドポイント形式

```
POST   https://discord.com/api/webhooks/{webhook.id}/{webhook.token}
PATCH  https://discord.com/api/webhooks/{webhook.id}/{webhook.token}/messages/{message.id}
DELETE https://discord.com/api/webhooks/{webhook.id}/{webhook.token}/messages/{message.id}
```

- 認証ヘッダーは不要。URL に含まれる `token` が認証情報となる。
- API バージョンは URL パスに含めない場合、現在のデフォルトバージョンが使われる（[API Versioning](https://discord.com/developers/docs/reference#api-versioning)）。

### メッセージ本文の必須条件

送信時は、以下のいずれか **1 つ以上** を指定する必要がある。


| フィールド        | 説明               |
| ------------ | ---------------- |
| `content`    | テキスト本文           |
| `embeds`     | リッチ埋め込み（最大 10 件） |
| `components` | メッセージコンポーネント     |
| `files[n]`   | ファイル添付           |
| `poll`       | 投票               |


`IS_COMPONENTS_V2` フラグを付与する場合は `components` のみ許可され、`content` / `embeds` / `files[n]` / `poll` を同時に送ると `400 Bad Request` になる。

### レート制限

- Webhook は `webhook_id` または `webhook_id + webhook_token` をキーにルート単位で制限される。
- `429 Too Many Requests` 時は `Retry-After` ヘッダーまたはレスポンス JSON の `retry_after` を参照して再送する。
- Webhook が `404` を返した場合、URL の無効化とみなし再送を止めること（繰り返しは IP 制限の原因になる）。

---

## ケース 1: テキストメッセージ送信

最も基本的な送信。`application/json` で POST する。

### リクエスト

```
POST /webhooks/{webhook.id}/{webhook.token}
Content-Type: application/json
```

```json
{
  "content": "Deploy complete"
}
```

### 制約


| 項目            | 値                   |
| ------------- | ------------------- |
| `content` 最大長 | 2000 文字             |
| 空送信           | 不可（`content` 単体の場合） |


### レスポンス


| 条件                          | ステータス            | ボディ                  |
| --------------------------- | ---------------- | -------------------- |
| デフォルト（`wait` 未指定 / `false`） | `204 No Content` | なし                   |
| `?wait=true`                | `200 OK`         | 作成された Message オブジェクト |


`wait=false`（デフォルト）の場合、保存に失敗したメッセージでもエラーが返らない点に注意する。

---

## ケース 2: 表示名・アバターの上書き

Webhook に設定されたデフォルト名・アバターを、送信時に一時的に変更できる。

### リクエスト

```json
{
  "content": "Deploy complete",
  "username": "CI Bot",
  "avatar_url": "https://example.com/avatar.png"
}
```

### フィールド


| フィールド        | 型      | 説明            |
| ------------ | ------ | ------------- |
| `username`   | string | 表示名の上書き       |
| `avatar_url` | string | アバター URL の上書き |


---

## ケース 3: TTS（読み上げ）メッセージ

### リクエスト

```json
{
  "content": "Alert: production is down",
  "tts": true
}
```


| フィールド | 型       | 説明                 |
| ----- | ------- | ------------------ |
| `tts` | boolean | `true` で TTS として送信 |


---

## ケース 4: Embed 付きメッセージ

リッチな表示向け。`content` と併用可能。

### リクエスト

```json
{
  "content": "Build finished",
  "embeds": [
    {
      "title": "CI Result",
      "description": "All checks passed.",
      "color": 3066993,
      "fields": [
        { "name": "Branch", "value": "main", "inline": true },
        { "name": "Commit", "value": "abc1234", "inline": true }
      ],
      "footer": { "text": "GitHub Actions" },
      "timestamp": "2026-10-08T12:00:00.000Z"
    }
  ]
}
```

### 制約


| 項目            | 値                                                                         |
| ------------- | ------------------------------------------------------------------------- |
| `embeds` 最大数  | 10                                                                        |
| `fields` 最大数  | 25（1 Embed あたり）                                                           |
| Webhook で設定不可 | `type`（常に `rich`）、`provider`、`video`、`height` / `width` / `proxy_url`（画像） |


### 利用可能な主なフィールド


| フィールド                 | 説明                              |
| --------------------- | ------------------------------- |
| `title`               | タイトル                            |
| `description`         | 説明文                             |
| `url`                 | タイトルリンク URL                     |
| `color`               | 左ボーダー色（10 進数 RGB）               |
| `timestamp`           | ISO8601 タイムスタンプ                 |
| `author`              | 作者情報（`name`, `url`, `icon_url`） |
| `footer`              | フッター（`text`, `icon_url`）        |
| `image` / `thumbnail` | 画像 URL（`url`）                   |
| `fields`              | 名前・値・`inline` の配列               |


---

## ケース 5: ファイル添付

画像やログファイルなどを添付する。`multipart/form-data` が必須。

### リクエスト

```
POST /webhooks/{webhook.id}/{webhook.token}
Content-Type: multipart/form-data
```

非ファイルパラメータは個別の form フィールド、または `payload_json` に JSON 文字列としてまとめて送る。

```json
// payload_json の例
{
  "content": "Screenshot attached",
  "attachments": [
    {
      "id": 0,
      "description": "Error screen",
      "filename": "error.png"
    }
  ]
}
```

```
files[0]: (binary, filename="error.png")
```

### 制約


| 項目                 | 値                                    |
| ------------------ | ------------------------------------ |
| ファイルサイズ上限          | デフォルト 20 MiB / ファイル（Nitro・Boost で増加） |
| `files[n]` の命名     | `files[0]`, `files[1]`, ... と一意にする   |
| `attachments[].id` | `files[n]` の `n` と対応するプレースホルダ        |


### Embed 内での添付参照

Embed の画像にアップロードしたファイルを使う場合:

```json
{
  "embeds": [{
    "image": { "url": "attachment://error.png" }
  }],
  "attachments": [{ "id": 0, "filename": "error.png" }]
}
```

Embed 画像として使える拡張子: `.jpg`, `.jpeg`, `.png`, `.webp`, `.gif`

---

## ケース 6: メンション制御

ユーザー生成文字列を `content` に含める場合、意図しないメンションを防ぐために `allowed_mentions` を指定する。

### Webhook のデフォルト動作

Webhook では `allowed_mentions` 未指定時、**ユーザーメンションのみ** がパースされる（通常メッセージとは異なる）。

```json
{ "parse": ["users"] }
```

### 全メンション抑制

```json
{
  "content": "@here Hello <@&1234>",
  "allowed_mentions": { "parse": [] }
}
```

### 特定 ID のみ許可

```json
{
  "content": "Hey <@123456789>",
  "allowed_mentions": {
    "users": ["123456789"]
  }
}
```

### フィールド


| フィールド          | 説明                                     |
| -------------- | -------------------------------------- |
| `parse`        | `"users"`, `"roles"`, `"everyone"` の配列 |
| `users`        | メンションを許可するユーザー ID（最大 100）              |
| `roles`        | メンションを許可するロール ID（最大 100）               |
| `replied_user` | リプライ先作者のメンション（リプライ時）                   |


---

## ケース 7: メッセージフラグ

`flags` はビットフィールド。Webhook 送信で設定可能な値:


| フラグ                      | 値                 | 説明                                   |
| ------------------------ | ----------------- | ------------------------------------ |
| `SUPPRESS_EMBEDS`        | `1 << 2` (4)      | URL プレビュー等の Embed 生成を抑制              |
| `SUPPRESS_NOTIFICATIONS` | `1 << 12` (4096)  | プッシュ・デスクトップ通知を抑制                     |
| `IS_COMPONENTS_V2`       | `1 << 15` (32768) | Components v2 メッセージ（`components` のみ） |


```json
{
  "content": "https://example.com",
  "flags": 4
}
```

---

## ケース 8: フォーラム / メディアチャンネルへの投稿

Webhook の紐づき先がフォーラムまたはメディアチャンネルの場合、スレッド指定が必須。

### 新規スレッド作成

```json
{
  "content": "New discussion topic",
  "thread_name": "Incident 2026-10-08",
  "applied_tags": ["1234567890123456789"]
}
```


| フィールド          | 説明            |
| -------------- | ------------- |
| `thread_name`  | 作成するスレッド名     |
| `applied_tags` | 適用するタグ ID の配列 |


### 既存スレッドへ送信

クエリパラメータで指定:

```
POST /webhooks/{id}/{token}?thread_id={thread.id}
```

アーカイブ済みスレッドは自動的にアンアーカイブされる。

---

## ケース 9: スレッド内への送信（通常チャンネル）

通常のテキストチャンネル内スレッドへ送る場合も `thread_id` クエリパラメータを使う。

```
POST /webhooks/{id}/{token}?thread_id={thread.id}
```

```json
{ "content": "Reply in thread" }
```

---

## ケース 10: 投票（Poll）

### リクエスト

```json
{
  "poll": {
    "question": { "text": "Deploy to production?" },
    "answers": [
      { "poll_media": { "text": "Yes" } },
      { "poll_media": { "text": "No" } }
    ],
    "duration": 24,
    "allow_multiselect": false
  }
}
```

### 制約


| 項目                  | 値                         |
| ------------------- | ------------------------- |
| 回答数                 | 最大 10                     |
| `question.text` 最大長 | 300 文字                    |
| 回答 `text` 最大長       | 55 文字                     |
| `duration`          | 時間単位、最大 32 日（デフォルト 24 時間） |


作成後の Poll メッセージは編集不可。

---

## ケース 11: コンポーネント（ボタン等）

### 非 Application-owned Webhook

インタラクティブコンポーネントは送信不可。非インタラクティブのみ `with_components=true` で有効化。

```
POST /webhooks/{id}/{token}?with_components=true
```

```json
{
  "content": "Choose an option",
  "components": [ /* ... */ ]
}
```

### Application-owned Webhook

`with_components` なしでも全コンポーネントを送信可能。

---

## ケース 12: 送信結果の取得（wait パラメータ）

メッセージ ID を後続操作（編集・削除）に使う場合、`wait=true` を付与する。

```
POST /webhooks/{id}/{token}?wait=true
```

レスポンス例（`200 OK`）:

```json
{
  "id": "1234567890123456789",
  "channel_id": "...",
  "content": "Deploy complete",
  ...
}
```


| `wait` 値          | デフォルト   | 挙動             |
| ----------------- | ------- | -------------- |
| Execute Webhook   | `false` | 非同期、本文なし `204` |
| Slack / GitHub 互換 | `true`  | 同期、Message 返却  |


---

## ケース 13: 送信済みメッセージの編集

同一 Webhook トークンで送ったメッセージのみ編集可能。

### リクエスト

```
PATCH /webhooks/{webhook.id}/{webhook.token}/messages/{message.id}
```

スレッド内の場合:

```
PATCH /webhooks/{webhook.id}/{webhook.token}/messages/{message.id}?thread_id={thread.id}
```

```json
{
  "content": "Deploy complete (updated)",
  "embeds": [
    { "title": "Status", "description": "Rolled back" }
  ]
}
```

### 注意点

- 全パラメータは任意かつ nullable。
- `content` 編集時、`allowed_mentions` 未指定ならデフォルトパース規則が再適用される。
- ファイル編集は `multipart/form-data`。`attachments` 配列に残すファイルをすべて列挙する（API v10 以降）。
- `IS_COMPONENTS_V2` への変更時は、既存の `content` / `embeds` / `poll` を `null` または `[]` にリセットする必要がある。

---

## ケース 14: 送信済みメッセージの削除

```
DELETE /webhooks/{webhook.id}/{webhook.token}/messages/{message.id}
```

スレッド内:

```
DELETE /webhooks/{webhook.id}/{webhook.token}/messages/{message.id}?thread_id={thread.id}
```

成功時: `204 No Content`

---

## ケース 15: Slack 互換 Webhook

Slack Incoming Webhook 形式のペイロードをそのまま送れるエンドポイント。

```
POST /webhooks/{webhook.id}/{webhook.token}/slack
```

### クエリパラメータ


| パラメータ       | 説明         | デフォルト  |
| ----------- | ---------- | ------ |
| `wait`      | 送信確認を待つ    | `true` |
| `thread_id` | 送信先スレッド ID | —      |


### 非対応の Slack フィールド

`channel`, `icon_emoji`, `mrkdwn`, `mrkdwn_in` はサポートされない。

---

## ケース 16: GitHub 互換 Webhook

GitHub リポジトリの Webhook 設定で Payload URL に指定する形式。GitHub からの POST を Discord が変換して表示する。

```
POST /webhooks/{webhook.id}/{webhook.token}/github
```

### クエリパラメータ


| パラメータ       | 説明         | デフォルト  |
| ----------- | ---------- | ------ |
| `wait`      | 送信確認を待つ    | `true` |
| `thread_id` | 送信先スレッド ID | —      |


### 対応 GitHub イベント

`commit_comment`, `create`, `delete`, `fork`, `issue_comment`, `issues`, `member`, `public`, `pull_request`, `pull_request_review`, `pull_request_review_comment`, `push`, `release`, `watch`, `check_run`, `check_suite`, `discussion`, `discussion_comment`

---

## エラーハンドリング早見表


| ステータス | 意味                  | 対処                       |
| ----- | ------------------- | ------------------------ |
| `204` | 送信成功（`wait=false`）  | —                        |
| `200` | 送信成功（`wait=true`）   | レスポンスから `message.id` を保持 |
| `400` | 不正なリクエストボディ         | フィールド制約・必須条件を確認          |
| `401` | トークン不正              | Webhook URL を再確認         |
| `404` | Webhook またはメッセージ不存在 | 再送を停止                    |
| `429` | レート制限               | `retry_after` 秒後に再送      |


