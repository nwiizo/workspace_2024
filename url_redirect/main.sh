#!/bin/bash

# URLを格納したファイルのパス
url_file="urls.txt"

# URLファイルを読み込む
while IFS= read -r url
do
    # リダイレクト先のURLを取得
    redirected_url=$(curl -Ls -o /dev/null -w %{url_effective} "$url")
    
    # タイトルを抽出
    title=$(curl -Ls "$redirected_url" | grep -oE '<title>(.*)</title>' | sed 's/<title>\(.*\)<\/title>/\1/')
    
    # Markdownリンクを出力
    echo "- [$title]($redirected_url)"
done < "$url_file"
