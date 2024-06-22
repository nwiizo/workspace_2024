# 記事について

記事を書いてます。こいつのサンプルリポジトリです。
https://syu-m-5151.hatenablog.com/entry/2024/06/21/135855

# Kind Go Development Examples

このリポジトリは、Kind (Kubernetes in Docker) を使用したGoアプリケーション開発の例を提供します。Skaffoldとの統合による高速な開発サイクルの実現と、マイクロサービスアーキテクチャのシミュレーションを含んでいます。

## 前提条件

- [Go](https://golang.org/doc/install)
- [Docker](https://docs.docker.com/get-docker/)
- [Kind](https://kind.sigs.k8s.io/docs/user/quick-start/#installation)
- [kubectl](https://kubernetes.io/docs/tasks/tools/)
- [Skaffold](https://skaffold.dev/docs/install/)

## 開発ワークフロー

1. コードを変更します。
2. 変更を保存します。
3. Skaffoldが自動的に以下を行います：
   - 変更を検知
   - 新しいDockerイメージをビルド
   - ビルドしたイメージをKindクラスタにロード
   - アプリケーションを再デプロイ

## 注意事項

- このリポジトリはローカル開発とテスト用です。本番環境への移行時には適切な調整が必要です。
- Kindクラスタを削除するには `kind delete cluster` を使用してください。
