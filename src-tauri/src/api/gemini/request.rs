use chrono::{DateTime, Utc};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Todo {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub issues: Option<Vec<Issue>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Issue {
    pub title: Option<String>,
    pub description: Option<String>,
    pub estimated_working_hours: Option<i16>,
}

// request to Gemini API
// parse response
pub async fn for_disassemble(str: &str) -> Result<Value, String> {
    let request_content = format!(
        r##"以下は現在直面している対応するべき問題または事象です。添付した指定構造に分解・分類し、TODO及びTaskに因子分解し日本語で出力してください。

構造化出力指定型各フィールドの説明:
- Todo: 事象のタイトル、概要、関連するIssueのリスト、作成日時を含む。
    - title: 事象のタイトル（短く端的に
    - summary: 事象の概要（要約
    - issues: 事象に関連するIssueのリスト
        - title: Issueの名称
        - description: Issueの対処法・問題解決法などの説明
        - estimated_working_hours: Issueの見積もり作業時間

いずれも大切なフィールドです。
フィールドが満ちていることを期待します。

対象: 
- デジタル非ネイティブの日本語話者
- 非プログラマ
- 役割: ディレクター
- 具体的な職務: 窓口、折衝、判断や評価、知識や話術差異を翻訳する業務が多い
- 部下に仕事を割り振り、進捗を管理する



出力時の注意点:
- **重複出力**: 重複出力、または同じ内容の出力は避けてください。
- **構造化**: Raw JSONとして構造化されたデータを期待します。


以下本文: 
```
{}
```"##,
        str
    );

    info!("{}", request_content);

    match request_to_gemini_api(&request_content).await {
        Ok(s) => {
            info!("{:?}", s.clone());
            let serded_value: Todo = match serde_json::from_str(s.clone().as_str()) {
                Ok(v) => v,
                Err(e) => {
                    error!("parsed error: {:?}, {:?}", e, s);
                    return Err(e.to_string());
                }
            };
            info!("serded: {:?}", serded_value);

            Ok(json!(serded_value))
        }
        Err(e) => {
            error!("error: {:?}", e);
            Err(e.to_string())
        }
    }
}

async fn request_to_gemini_api(str: &str) -> Result<String, String> {
    let models = std::env::var("GEMINI_MIDEL").unwrap_or_default();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        models,
        std::env::var("GEMINI_API_TOKEN").unwrap_or_default()
    );

    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let body = json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": str
                    }
                ]
            }
        ],
        "generationConfig": {
            "temperature": 0.1,
            "maxOutputTokens": 2048,
            "topK": 1,
            "topP": 0.1,
            "response_mime_type": "application/json",
            "response_schema": {
                "type": "OBJECT",
                "properties": { // ルートオブジェクトのプロパティを定義
                    "title": {
                        "type": "STRING",
                        "description": "The main title of the output." // 説明を追加すると良い
                     },
                    "summary": {
                        "type": "STRING",
                        "description": "A brief summary."
                     },
                    "issues": {
                        "type": "ARRAY",
                        "description": "A list of identified issues.",
                        "items": {
                            "type": "OBJECT",
                            "properties": {
                                "title": {
                                    "type": "STRING",
                                    "description": "Title of the issue."
                                 },
                                "description": {
                                    "type": "STRING",
                                    "description": "Detailed description of the issue."
                                 },
                                "estimated_working_hours": {
                                    "type": "INTEGER",
                                    "description": "Estimated hours to resolve the issue."
                                 }
                            },
                            "required": ["title", "description"], // 例: issues内の必須項目を指定
                            "propertyOrdering": ["title", "description", "estimated_working_hours"] // 例: issues内の順序を指定
                        }
                    }
                },
                "required": ["title", "summary", "issues"], // 例: ルートの必須項目を指定
                "propertyOrdering": ["title", "summary", "issues"] // 例: ルートの順序を指定
            }
        }
    });

    let res = match client.post(&url).headers(headers).json(&body).send().await {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    // 1. JSON文字列を `serde_json::Value` にパース
    match res.json::<Value>().await {
        Ok(value) => {
            // 2. `candidates` 配列を取得 (Option<&Value> として)
            //    `value["candidates"]` は存在しないキーだと panic する可能性があるため、
            //    `get()` を使うのが安全。
            let candidates_value = value.get("candidates");

            // 3. `candidates` が配列であることを確認し、最初の要素を取得
            //    `and_then` を使うとネストを避けられる
            let first_candidate_value = candidates_value
                .and_then(|candidates| candidates.as_array()) // candidates が配列なら Some(&Vec<Value>)
                .and_then(|arr| arr.first()); // 配列が空でなければ Some(&Value)

            // 4. 最初の候補 (candidate) がオブジェクトであることを確認し、`content` を取得
            let content_value = first_candidate_value
                .and_then(|candidate| candidate.as_object()) // candidate がオブジェクトなら Some(&Map<String, Value>)
                .and_then(|obj| obj.get("content")); // オブジェクトに "content" キーがあれば Some(&Value)

            // 5. `content` の値が見つかったかどうかで分岐
            match content_value {
                Some(content) => {
                    // --- さらに content の中の text を取得する例 ---
                    // content["parts"][0]["text"] のようなアクセスを安全に行う

                    let text = content
                        .get("parts") // Some(&Value) (parts 配列) or None
                        .and_then(|parts| parts.as_array()) // Some(&Vec<Value>) or None
                        .and_then(|arr| arr.first()) // Some(&Value) (最初の part オブジェクト) or None
                        .and_then(|part| part.as_object()) // Some(&Map<String, Value>) or None
                        .and_then(|obj| obj.get("text")) // Some(&Value) (text 文字列) or None
                        .and_then(|text_val| text_val.as_str()); // Some(&str) or None

                    match text {
                        Some(_) => (),
                        None => println!(
                            "`content.parts[0].text` が見つからないか、文字列ではありませんでした。"
                        ),
                    }

                    // 全ての part の text を連結する場合
                    let all_texts =
                        content
                            .get("parts")
                            .and_then(|parts| parts.as_array())
                            .map(|arr| {
                                // Option::map を使う
                                arr.iter()
                                    .filter_map(|part| part.get("text")) // 各 part から "text" を取得 (Option<&Value>)
                                    .filter_map(|text_val| text_val.as_str()) // 文字列なら &str に変換
                                    .collect::<Vec<&str>>() // &str の Vec を作成
                            }); // 結果は Option<Vec<&str>>

                    if let Some(texts) = all_texts {
                        Ok(texts.join(""))
                    } else {
                        println!("`content.parts` が配列でないか、見つかりませんでした。");
                        Err("`content.parts` が配列でないか、見つかりませんでした。".to_string())
                    }

                    // 必要であれば content オブジェクトを所有権付きでコピー
                    // let owned_content: Value = content.clone();
                }
                None => {
                    println!(
                        "`candidates` 配列の最初の要素、またはその中の `content` フィールドが見つかりませんでした。  {}",
                        value
                    );
                    Err("`candidates` 配列の最初の要素、またはその中の `content` フィールドが見つかりませんでした。".to_string())
                }
            }
        }
        Err(e) => {
            eprintln!("JSONのパースに失敗しました: {}", e);
            Err(e.to_string())
        }
    }
}
