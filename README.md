# tumugi

tumugi は、複数のAI言語モデルプロバイダー（OpenAI、Anthropic、Groq）との対話を管理するためのRustライブラリです。効率的な会話履歴管理と柔軟なAPIインターフェースを提供します。

## 主な機能

- 複数のAIプロバイダー（OpenAI、Anthropic、Groq）に対応
- 効率的な会話履歴管理（ConversationBufferWindowMemory）
- 非同期APIリクエスト処理
- RESTful APIインターフェース（axumフレームワーク使用）
- カスタムエラー処理

## 依存関係

- Rust 1.54以上
- tokio (非同期ランタイム)
- reqwest (HTTPクライアント)
- serde (シリアライゼーション/デシリアライゼーション)
- axum (Webフレームワーク)
- dotenv (環境変数管理)

## セットアップ

1. リポジトリをクローンします：

```bash
git clone https://github.com/yourusername/tumugi.git
cd tumugi
```

2. 必要な環境変数を設定します。プロジェクトルートに `.env` ファイルを作成し、以下の内容を追加します：

```
GROQ_API_KEY=your_groq_api_key_here
```

3. 依存関係をインストールし、プロジェクトをビルドします：

```bash
cargo build
```

## 使用方法

### ライブラリとして使用

`Cargo.toml` に以下を追加します：

```toml
[dependencies]
tumugi = { git = "https://github.com/yourusername/tumugi.git" }
```

基本的な使用例：

```rust
use tumugi::client::{ChatMessage, ChatRequest, Client};
use tumugi::memory::ConversationBufferWindowMemory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("GROQ_API_KEY").expect("API_KEY must be set");
    let client = Client::new(api_key, "llama3-8b-8192".to_string());
    
    let mut memory = ConversationBufferWindowMemory::new(5);
    memory.add_message(ChatMessage::user("Hello, how are you?".to_string()));

    let chat_req = ChatRequest::new("llama3-8b-8192".to_string(), memory.get_messages());
    let chat_res = client.create_chat_completion(chat_req).await?;
    
    println!("Response: {}", chat_res.choices[0].message.content);
    
    Ok(())
}
```

### サンプルの実行

提供されているサンプルを実行するには：

```bash
cargo run --example conversation_buffer_window_memory
cargo run --example chatreq_test
cargo run --example use_memory_on_restapi
```

## テスト

テストを実行するには：

```bash
cargo test
```

出力を表示してテストを実行するには：

```bash
cargo test -- --nocapture
```

## ライセンス

このプロジェクトは MIT ライセンスまたは Apache License 2.0 の下で提供されています。詳細については `LICENSE` ファイルを参照してください。

## 貢献

バグ報告、機能リクエスト、プルリクエストを歓迎します。大きな変更を行う前に、まずイシューを開いて変更内容について議論することをお勧めします。

## 注意事項

このライブラリは開発中であり、APIは変更される可能性があります。本番環境での使用には注意してください。