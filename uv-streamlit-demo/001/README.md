# uvパッケージマネージャーとDocker、Streamlitを使った開発環境の構築

## はじめに

Pythonの開発環境には新しい選択肢が増えています。最近注目を集めているパッケージマネージャーの`uv`を使用して、Streamlitアプリケーションを構築し、Dockerコンテナで実行する方法について説明します。

- [Production-ready Python Docker Containers with uv](https://hynek.me/articles/docker-uv/) などの記事で紹介されているように、`uv`を使用することで、Pythonのパッケージ管理を効率化できます。

## プロジェクトの構造

```
project/
├── Dockerfile      # Dockerコンテナの定義
├── README.md       # プロジェクトの説明ドキュメント
├── app.py          # Streamlitアプリケーションのコード
├── pyproject.toml  # プロジェクトの依存関係定義
└── uv.lock         # 依存パッケージのバージョンをロック
```

## 各ファイルの実装

### 1. Streamlitアプリケーション (app.py)

```python
import streamlit as st

def main():
    st.title("Hello World!")
    st.write("Welcome to the Streamlit version of the UV demo app!")
    
    if st.button("Click me!"):
        st.balloons()
        
    st.sidebar.markdown("""
    ### About
    This is a simple Streamlit app demonstrating UV package management.
    """)

if __name__ == "__main__":
    main()
```

### 2. Dockerファイル (Dockerfile)

```dockerfile
ARG PYTHON_VERSION="3.13.0"

# uvの公式docker image
FROM ghcr.io/astral-sh/uv:python${PYTHON_VERSION%.*}-bookworm-slim as build

ENV UV_LINK_MODE=copy \
    UV_COMPILE_BYTECODE=1 \
    UV_PYTHON_DOWNLOADS=never \
    UV_PYTHON=python${PYTHON_VERSION%.*}

WORKDIR /app

# 依存関係ファイルのみをまずコピー
COPY ./pyproject.toml ./uv.lock ./

# パッケージのインストール
RUN uv sync --frozen --no-install-project

# アプリケーションコードをコピー
COPY app.py .

FROM python:${PYTHON_VERSION}-slim-bookworm

ENV PATH=/app/.venv/bin:$PATH

WORKDIR /app

COPY --from=build /app /app

EXPOSE 8501

CMD [ "streamlit", "run", "app.py" ]
```

### 3. プロジェクト設定 (pyproject.toml)

```toml
[project]
name = "uv-streamlit-demo"
version = "0.1.0"
description = "A simple Streamlit app using UV package management"
requires-python = ">=3.12"
dependencies = [
    "streamlit>=1.39.0",
]
```

### 4. README.md

プロジェクトのセットアップと実行方法を説明するドキュメントを作成します。

# UV Streamlit Demo

この、プロジェクトは、UVパッケージマネージャーをStreamlitとDockerで使う方法を示しています。

## Setup

### 1. Install UV:
そのほかにも、[公式ドキュメント](https://docs.astral.sh/uv/getting-started/installation/)を参照してください。

```bash
pip install uv
```

### 2. Initialize the project:
新しいPythonプロジェクトを作成します。

```bash
uv init
```

### 3. Install dependencies:
ローカルのPythonのバージョンとDockerfileのPythonバージョンが一致するようにしてください。一致させられない場合は、`pyproject.toml`と`uv.lock`を修正してください。
```bash
uv add streamlit
```

## Running the Application

### With Docker
実際にdockerイメージをビルドして、コンテナを実行します。


1. Build the image:
```bash
docker build -t uv-streamlit:latest .
```

2. Run the container:
```bash
docker run --rm --name test -p 8501:8501 uv-streamlit:latest
```

3. Open http://localhost:8501 in your browser

### Without Docker

1. Install dependencies:
```bash
uv pip install -r requirements.txt
```

2. Run the app:
```bash
streamlit run app.py
```


## セットアップ実行手順

### 1. プロジェクトの初期化

```bash
# プロジェクトディレクトリの作成
mkdir uv-streamlit-demo
cd uv-streamlit-demo

# uvの初期化
uv init

# Streamlitの追加
uv add streamlit
```

これにより、以下のファイルが生成されます：
- `pyproject.toml`: プロジェクトの依存関係定義
- `uv.lock`: 依存パッケージのバージョン固定

### 2. Dockerイメージのビルドと実行

```bash
# イメージのビルド
docker build -t uv-streamlit:latest .

# コンテナの実行
docker run --rm --name test -p 8501:8501 uv-streamlit:latest
```

## 実装のポイント

### 1. 依存関係の管理

- `uv.lock`ファイルにより、全ての依存パッケージのバージョンが固定される
- これにより、開発環境と本番環境で同じバージョンのパッケージが使用される
- `--frozen`オプションで、ロックファイルの内容を厳密に守る

### 2. マルチステージビルド

1. ビルドステージ（uvイメージ）:
   - 依存関係のインストール
   - ロックファイルの使用
   - アプリケーションコードのコピー

2. 実行ステージ（Pythonイメージ）:
   - 必要なファイルのみをコピー
   - 最小限の実行環境

### 3. 環境変数の活用

Dockerfileでの重要な環境変数：

- `UV_LINK_MODE=copy`: パッケージのインストール方式
- `UV_COMPILE_BYTECODE=1`: パフォーマンス最適化
- `UV_PYTHON_DOWNLOADS=never`: コンテナ環境の活用
- `UV_PYTHON`: Pythonバージョンの指定

## メリット

1. **開発環境の一貫性**
   - ロックファイルによる厳密なバージョン管理
   - Dockerによる環境の標準化
   - READMEによる明確なセットアップ手順

2. **効率的なビルドプロセス**
   - マルチステージビルドによる最適化
   - uvの高速パッケージインストール
   - 依存関係の階層的な管理

3. **保守性の向上**
   - 明確プロジェクト構造
   - ドキュメント化されたセットアップ手順
   - バージョン管理された依存関係

## まとめ

uvを使用したStreamlitアプリケーションの開発環境は、効率的で再現性の高い開発を可能にします。特に以下のような場合に効果を発揮します：

- チーム開発での環境の統一が必要な場合
- CI/CDパイプラインでの利用
- 本番環境での確実なデプロイ

プロジェクトの構造をシンプルに保ちながら、必要な要素を過不足なく含めることで、メンテナンス性の高い開発環境を実現できます。

## 参考リンク

- [UV公式ドキュメント](https://github.com/astral-sh/uv)
- [Streamlit公式ドキュメント](https://docs.streamlit.io/)
- [Docker公式ドキュメント](https://docs.docker.com/)
