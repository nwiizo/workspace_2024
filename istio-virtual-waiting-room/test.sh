#!/bin/bash

# 同時接続数
CONCURRENT=10

# 総リクエスト数
TOTAL_REQUESTS=50

# 結果を保存するディレクトリ
RESULTS_DIR="access_test_results"
mkdir -p "$RESULTS_DIR"

# 関数: 単一のリクエストを実行し、結果を記録
make_request() {
    local id=$1
    local start_time=$(date +%s.%N)
    local http_code=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080)
    local end_time=$(date +%s.%N)
    local duration=$(echo "$end_time - $start_time" | bc)
    echo "$id,$http_code,$duration" >> "$RESULTS_DIR/results.csv"
}

# CSVヘッダーの作成
echo "RequestID,HTTPCode,Duration" > "$RESULTS_DIR/results.csv"

# 同時リクエストの実行
for i in $(seq 1 $TOTAL_REQUESTS); do
    make_request $i &
    
    # 同時接続数を制御
    if (( i % CONCURRENT == 0 )); then
        wait
    fi
done

# 残りのプロセスを待機
wait

echo "All requests completed. Results saved in $RESULTS_DIR/results.csv"

# 結果の分析と表示
echo "===============================
テスト結果サマリー
==============================="
echo "総リクエスト数: $TOTAL_REQUESTS"
echo "同時接続数: $CONCURRENT"

echo -e "\nHTTPステータスコード分布:"
status_codes=$(sort "$RESULTS_DIR/results.csv" | cut -d',' -f2 | sort | uniq -c | sort -nr)
total_success=$(echo "$status_codes" | grep " 200" | awk '{print $1}')
total_success=${total_success:-0}
echo "$status_codes" | while read count code; do
    if [ "$code" != "HTTPCode" ]; then
        percentage=$(echo "scale=2; $count / $TOTAL_REQUESTS * 100" | bc)
        printf "%s: %s (%.2f%%)\n" "$code" "$count" "$percentage"
        
        # 簡単な視覚化
        bar=$(printf '%0.s#' $(seq 1 $(echo "$percentage/2" | bc)))
        printf "  %s\n" "$bar"
    fi
done

success_rate=$(echo "scale=2; $total_success / $TOTAL_REQUESTS * 100" | bc)
echo -e "\n成功率（200 OKの割合）: ${success_rate}%"

echo -e "\n応答時間統計:"
awk -F',' '
    NR>1 {
        sum+=$3; 
        sumsq+=$3*$3; 
        if(NR==2 || $3<min) min=$3; 
        if(NR==2 || $3>max) max=$3;
    } 
    END {
        avg=sum/NR; 
        std=sqrt(sumsq/NR - avg*avg);
        printf "最小: %.2f秒\n", min;
        printf "最大: %.2f秒\n", max;
        printf "平均: %.2f秒\n", avg;
        printf "標準偏差: %.2f秒\n", std;
    }
' "$RESULTS_DIR/results.csv"

echo -e "\n注: 詳細な結果は $RESULTS_DIR/results.csv に保存されています。"
