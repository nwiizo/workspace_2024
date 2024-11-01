# Dockerfile
ARG PYTHON_VERSION="3.13.0"

# uvの公式docker image
FROM ghcr.io/astral-sh/uv:python${PYTHON_VERSION%.*}-bookworm-slim as build

# UV_LINK_MODE wheelからパッケージをコピーしてインストールする
# UV_COMPILE_BYTECODE インストール後にソースファイルをバイトコードにコンパイルする
# UV_PYTHON_DOWNLOADS コンテナのpython環境を使うのでuvにはpythonをダウンロードさせない
# UV_PYTHON uvを操作する際に利用するpythonを指定する
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
