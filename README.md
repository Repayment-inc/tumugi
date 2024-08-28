# tumugi

tumugiは、Rust言語で実装されたAIエージェントシステム構築ライブラリです。PythonのLangchainに相当する機能を提供することを目標としつつ、Rustの並列処理能力を活かした独自の拡張機能を備えています。複数のAI言語モデルプロバイダー（OpenAI、Anthropic、Groq）との統合、効率的な会話履歴管理、Self-Refine機能、そして革新的な適応型マルチロールジェネレータを提供します。

<!-- Badges -->
<table align="center">
    <tr>
        <td>License</td>
        <td>Lang</td>
        <!-- <td>Packages</td> -->
        <td>AI</td>
        <td>Github</td>
    </tr>
    <tr>
        <td>
            <a href="./LICENSE-MIT">
                <img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat">
            </a>
        </td>
        <td>
            <img src="https://img.shields.io/badge/Rust-000000.svg?logo=rust">
            <!-- <img src="https://img.shields.io/badge/-Rust-555.svg?logo=rust&style=flat"> -->
        </td>
        <!-- <td>
            <img src="https://img.shields.io/badge/tokio-v1.0-orange.svg">
            <img src="https://img.shields.io/badge/axum-v0.7.5-orange.svg">
        </td> -->
        <td>
            <img src="https://img.shields.io/badge/OpenAI-412991.svg?logo=openai&style=flat">
            <img src="https://img.shields.io/badge/Anthropic-191919.svg?logo=anthropic&style=flat">
            <br>
            <img src="https://img.shields.io/badge/Groq-orange.svg">
            <!-- <img src="https://img.shields.io/badge/Gemini-191919.svg?logo=google-gemini&style=flat"> -->
        </td>
        <td>
            <a href="https://github.com/Repayment-inc/tumugi">
                <img src="https://img.shields.io/github/stars/Repayment-inc/tumugi.svg?style=flat&logo=github&colorB=deeppink&label=stars">
            </a>
            <a href="https://github.com/Repayment-inc/tumugi">
                <img src="https://img.shields.io/github/forks/Repayment-inc/tumugi.svg">
            </a>
        </td>
    </tr>
</table>


## 主な機能

1. **適応型マルチロールジェネレータ (Adaptive Multi-Role Generator)**:
   - 複雑な目標に対し、司令塔エージェントが必要な役割を動的に特定・生成します。
   - 各役割に特化したサブエージェントを自動生成し、並列で稼働させます。
   - Rustの並列処理能力を最大限に活用し、複雑なタスクを効率的に解決します。
   - 目標達成に向けて、役割、チーム、タスクを柔軟に再構成します。
   

2. **AIマルチプロバイダ対応**:
   - OpenAI、Anthropic、Groqなど、複数のAI言語モデルプロバイダーに対応しています。
   - 統一されたインターフェースを通じて、異なるプロバイダーのモデルを簡単に切り替えられます。

3. **効率的な会話履歴管理**:
   - ConversationBufferWindowMemoryを使用して、AIとの対話の文脈を維持します。
   - メモリ使用を最適化しつつ、重要な情報を保持します。

4. **自己改善機能 (Self-Refine)**:
   - AIが生成した内容を自動的に評価し、改善するプロセスを提供します。
   - 生成された文章の品質を向上させ、より洗練された出力を実現します。

5. **非同期APIリクエスト処理**:
   - Rustの非同期機能を活用し、効率的なAPIコミュニケーションを実現します。

6. **RESTful APIインターフェース**:
   - axumフレームワークを使用し、高性能なWebAPIを提供できます。

7. **カスタムエラー処理**:
   - 詳細なエラー情報を提供し、デバッグと問題解決を容易にします。

## なぜtumugiを選ぶのか？

- **Rustの強みを活かした設計**: 並列処理と非同期処理を最大限に活用し、高性能で効率的なAIシステムを構築できます。
- **柔軟性と拡張性**: 適応型マルチロールジェネレータにより、一つの目標を達成するために必要な要素を、役割、チーム、タスクに分解し、動的にエージェントを構成できます。
- **統合されたエコシステム**: AIマルチプロバイダ対応、会話履歴管理、自己改善機能など、AIアプリケーション開発に必要な要素が統合されています。
- **高度な並列処理**: 複数のAIエージェントを効率的に同時稼働させ、複雑な問題解決を加速します。
- **最新の研究に基づく実装**: 最新のAI研究成果を取り入れ、常に進化するライブラリです。

## セットアップ

1. リポジトリをクローンします：

```bash
git clone https://github.com/yourusername/tumugi.git
cd tumugi
```

2. 必要な環境変数を設定します。プロジェクトルートに `.env` ファイルを作成し、以下の内容を追加します：

```
OPENAI_API_KEY=your_openai_api_key_here
ANTHROPIC_API_KEY=your_anthropic_api_key_here
GROQ_API_KEY=your_groq_api_key_here
```

3. 依存関係をインストールし、プロジェクトをビルドします：

```bash
cargo build
```

## 使用例

### 適応型マルチロールジェネレータの使用

```rust
use tumugi::adaptive_multi_role::AdaptiveMultiRoleSystem;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").expect("API_KEY must be set");
    let model = "gpt-4o-mini".to_string();

    let system = AdaptiveMultiRoleSystem::new(api_key, model)?;

    let goal = "持続可能なスマートシティの設計と実装計画の策定";
    let result = system.process(goal).await?;

    println!("最終結果: {}", result);

    Ok(())
}
```

この例では、複雑な「持続可能なスマートシティの設計と実装計画の策定」という目標に対して、AdaptiveMultiRoleSystemが自動的に必要な役割（都市計画専門家、環境科学者、ITアーキテクト、エネルギー管理専門家など）を特定し、それぞれの役割に応じたサブエージェントを並列で生成・実行します。これにより、多面的な専門知識を統合し、包括的な解決策を効率的に生成します。

### Self-Refine機能の使用

```rust
use tumugi::self_refine::SelfRefineSystem;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let model = "gpt-4o-mini".to_string();
    let iterations = 2;
    let memory_size = 10;

    let mut self_refine_system = SelfRefineSystem::new(api_key, model, iterations, memory_size)?;

    let prompt = "AIの倫理的な使用と社会への影響について、500字程度の論述を作成してください。";
    let result = self_refine_system.process(prompt).await?;

    println!("\n最終結果:\n{}", result);

    Ok(())
}
```

### その他の例

- [examples/chat_completions_api.rs](examples/chat_completions_api.rs) - チャット完了APIの利用。
- [examples/conversation_buffer_window_memory.rs](examples/conversation_buffer_window_memory.rs) - 会話履歴のメモリ保存。
- [examples/use_memory_on_restapi.rs](examples/use_memory_on_restapi.rs) -  Web API フレームワーク `Axum` での利用。

## 貢献

バグ報告、機能リクエスト、プルリクエストを歓迎します。

## ライセンス

このプロジェクトは MIT ライセンス及びApache License 2.0 の下で提供されています。

## 注意事項

このライブラリは活発に開発中であり、APIは変更される可能性があります。本番環境での使用には注意してください。


## 開発予定の機能

1. プロンプトテンプレート機能
    - 再利用可能で柔軟なプロンプト設計を可能にする機能。

2. MRKL [(MRKL Systems)](https://arxiv.org/pdf/2205.00445) 
    - 複数のAIモデルや外部ツールを組み合わせ、より高度な推論と問題解決を行う機能。
    
3. ReAct [(REACT: SYNERGIZING REASONING AND ACTING IN LANGUAGE MODELS)](https://arxiv.org/pdf/2210.03629)
    - 言語モデルに推論と行動を組み合わせる能力を付与し、より動的で適応性の高いAIエージェントを実現する機能。
