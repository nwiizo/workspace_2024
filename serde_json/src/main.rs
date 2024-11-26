use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;

// ユーザーの基本構造体
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    age: u32,
    email: String,
    is_active: bool,
    // オプショナルなフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
}

// カスタムエラー型の定義
#[derive(Debug)]
enum UserError {
    ParseError(serde_json::Error), // JSONパースエラー
    ValidationError(String),       // バリデーションエラー
    DatabaseError(String),         // DB操作エラー
    IoError(io::Error),            // ファイル操作エラー
}

// serde_json::ErrorからUserErrorへの変換を実装
impl From<serde_json::Error> for UserError {
    fn from(err: serde_json::Error) -> UserError {
        UserError::ParseError(err)
    }
}

// io::ErrorからUserErrorへの変換を実装
impl From<io::Error> for UserError {
    fn from(err: io::Error) -> UserError {
        UserError::IoError(err)
    }
}

// std::error::Errorトレイトの実装
impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::ParseError(e) => write!(f, "Parse error: {}", e),
            UserError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            UserError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            UserError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for UserError {}

// Userの実装
impl User {
    // バリデーションメソッド
    fn validate(&self) -> Result<(), UserError> {
        if self.name.is_empty() {
            return Err(UserError::ValidationError(
                "Name cannot be empty".to_string(),
            ));
        }
        if self.age > 150 {
            return Err(UserError::ValidationError("Invalid age".to_string()));
        }
        if !self.email.contains('@') {
            return Err(UserError::ValidationError(
                "Invalid email format".to_string(),
            ));
        }
        Ok(())
    }
}

// 基本的なJSONパース関数
fn parse_user(json_str: &str) -> Result<User, serde_json::Error> {
    // map_errを使用してエラーをログ出力
    serde_json::from_str(json_str).map_err(|e| {
        println!("Error parsing JSON: {}", e);
        e // 元のエラーを返す
    })
}

// より詳細なエラーハンドリングを行う関数
fn process_user_data(json_str: &str) -> Result<User, UserError> {
    // JSONのパース
    let user: User = serde_json::from_str(json_str)?; // ?演算子でエラーを伝播

    // バリデーション
    user.validate()?; // ?演算子でエラーを伝播

    Ok(user)
}

// 複数ユーザーからの検索（Option型との組み合わせ）
fn find_user_by_id(json_str: &str, target_id: u32) -> Result<Option<User>, UserError> {
    // JSONから複数ユーザーパース
    let users: Vec<User> = serde_json::from_str(json_str)?;

    // 指定されたIDのユーザーを探す
    Ok(users.into_iter().find(|user| user.id == target_id))
}

// ファイル操作を含むエラーハンドリング
fn load_user_from_file(path: &str) -> Result<User, UserError> {
    // ファイルを読み込み
    let content = fs::read_to_string(path).map_err(|e| {
        eprintln!("Failed to read file {}: {}", path, e);
        UserError::IoError(e)
    })?;

    // JSONをパースしてUserを返す
    process_user_data(&content)
}

// ファイルへの保存
fn save_user_to_file(user: &User, path: &str) -> Result<(), UserError> {
    // UserをJSONに変換
    let json = serde_json::to_string_pretty(user).map_err(|e| {
        eprintln!("Failed to serialize user: {}", e);
        UserError::ParseError(e)
    })?;

    // ファイルに書き込み
    fs::write(path, json).map_err(|e| {
        eprintln!("Failed to write to file {}: {}", path, e);
        UserError::IoError(e)
    })?;

    Ok(())
}

fn main() {
    // 1. 有効なJSONの例
    let valid_json = r#"
        {
            "id": 1,
            "name": "John Doe",
            "age": 30,
            "email": "john@example.com",
            "is_active": true,
            "metadata": {
                "last_login": "2024-01-01",
                "location": "Tokyo"
            }
        }
    "#;

    // 2. 無効なJSONの例（バリデーションエラー）
    let invalid_json = r#"
        {
            "id": 2,
            "name": "",
            "age": 200,
            "email": "invalid-email",
            "is_active": true
        }
    "#;

    // 3. 複数ユーザーのJSONの例
    let users_json = r#"[
        {
            "id": 1,
            "name": "John Doe",
            "age": 30,
            "email": "john@example.com",
            "is_active": true
        },
        {
            "id": 2,
            "name": "Jane Doe",
            "age": 25,
            "email": "jane@example.com",
            "is_active": true
        }
    ]"#;

    // 4. 各種エラーハンドリングの実演
    println!("1. 基本的なパース:");
    match parse_user(valid_json) {
        Ok(user) => println!("成功: {:?}", user),
        Err(e) => println!("エラー: {}", e),
    }

    println!("\n2. バリデーション付きパース:");
    match process_user_data(invalid_json) {
        Ok(user) => println!("成功: {:?}", user),
        Err(e) => println!("エラー: {}", e),
    }

    println!("\n3. ユーザー検索:");
    match find_user_by_id(users_json, 1) {
        Ok(Some(user)) => println!("ユーザーが見つかりました: {:?}", user),
        Ok(None) => println!("ユーザーが見つかりません"),
        Err(e) => println!("エラー: {}", e),
    }

    println!("\n4. ファイル操作:");
    // 有効なユーザーをファイルに保存
    if let Ok(user) = parse_user(valid_json) {
        match save_user_to_file(&user, "user.json") {
            Ok(()) => println!("ユーザーを保存しました"),
            Err(e) => println!("保存エラー: {}", e),
        }

        // 保存したファイルから読み込み
        match load_user_from_file("user.json") {
            Ok(loaded_user) => println!("ロードしたユーザー: {:?}", loaded_user),
            Err(e) => println!("ロードエラー: {}", e),
        }
    }
}
