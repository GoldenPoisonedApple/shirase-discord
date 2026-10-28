# .envファイルを読み込む
# Makeの変数として扱う
# -include: ファイルが存在しない場合はエラーとしない
-include .env
export

# ドキュメントを生成してブラウザで開く
.PHONY: docs
docs:
	cargo doc --all-features --open

# ----------------------------
# チェック
# ----------------------------

# --all: ワークスペース全体
.PHONY: format
format:
	cargo fmt --all

# default は blocking のみ
# async のみ、および両方でも警告が出ないことを確認する
# --all-features: すべての機能をテスト
.PHONY: lint
lint:
	cargo clippy --all-targets -- --deny warnings
	cargo clippy --all-targets --no-default-features --features async -- --deny warnings
	cargo clippy --all-targets --all-features -- --deny warnings

# --allow-dirty: コミットしてなくてもいい
# --allow-no-vcs: バージョン管理システムがなくてもいい
.PHONY: lint-fix
lint-fix:
	cargo clippy --all-targets --all-features --fix --allow-dirty --allow-no-vcs

.PHONY : test
test:
	cargo test --all-targets
	cargo test --all-targets --no-default-features --features async
	cargo test --all-targets --all-features

# advisories: 脆弱性チェック
# licenses: ライセンスチェック
# bans: 禁止チェック
# sources: 依存元チェック
.PHONY: check
check:
	$(MAKE) format
	$(MAKE) lint
	$(MAKE) test
	cargo deny check


# チェック
.PHONY: ci
ci:
	$(MAKE) lint
	$(MAKE) test
	cargo fmt --all -- --check


# ----------------------------
# test
# ----------------------------
.PHONY: example
example:
	cargo run --example blocking_webhook

.PHONY: example-async
example-async:
	cargo run --example async_webhook --features async

# 送信テスト
.PHONY: send-test
send-test:
	curl -X POST -H "Content-Type: application/json" -d @test.json $(DISCORD_WEBHOOK_URL)

# <: 文字列としてフィールドに入れる
# @: ファイルとして送る
.PHONY: send-test-file
send-test-file:
	curl -vvv -X POST -F "payload_json=<test-file.json" \
	 -F "files[0]=@sample00.png;type=application/octet-stream" \
	 -F "files[1]=@sample01.png;type=application/octet-stream" \
	 $(DISCORD_WEBHOOK_URL)