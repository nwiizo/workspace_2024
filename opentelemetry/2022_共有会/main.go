// パッケージ main はエントリポイントを定義する。
package main

import (
	"context"
	"log"
	"net/http"

	"go.opentelemetry.io/contrib/instrumentation/net/http/otelhttp"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/exporters/otlp/otlptrace/otlptracegrpc"
	"go.opentelemetry.io/otel/sdk/resource"
	sdktrace "go.opentelemetry.io/otel/sdk/trace"
	semconv "go.opentelemetry.io/otel/semconv/v1.22.0"
)

func main() {
	// tracerProvider関数を使用してトレーサープロバイダを初期化する。
	tp, err := tracerProvider("otelhttp_client_trace")
	if err != nil {
		log.Fatal(err)
	}
	// main関数の終了時にトレーサープロバイダをシャットダウンする。
	defer func() { _ = tp.Shutdown(context.Background()) }()
	// グローバルトレーサープロバイダをットする。
	otel.SetTracerProvider(tp)

	// OpenTelemetryインストルメンテーションを使用したHTTPクライアントを作成する。
	client := &http.Client{Transport: otelhttp.NewTransport(http.DefaultTransport)}

	// トレースを開始するためのコンテキストを作成する。
	ctx := context.Background()
	ctx, span := otel.Tracer("main").Start(ctx, "main-span")
	// スパンを終了する。
	defer span.End()

	// トレースされたHTTPリクエストを作成する。
	req, err := http.NewRequestWithContext(ctx, "GET", "https://3-shake.com/", nil)
	if err != nil {
		log.Fatal(err)
	}

	// トレースされたHTTPリクエストを送信する。
	resp, err := client.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	// レスポンスのボディをクローズする。
	defer resp.Body.Close()
}

func tracerProvider(serviceName string) (*sdktrace.TracerProvider, error) {
	// OTLPエクスポーターを作成する。
	exp, err := otlptracegrpc.New(
		context.Background(),
		otlptracegrpc.WithEndpoint("localhost:4317"),
		otlptracegrpc.WithInsecure(),
		// 必要に応じて他の設定オプションを追加する。
		otlptracegrpc.WithHeaders(map[string]string{
			// 必要に応じてカスタムヘッダーを追加する。
			"Authorization": "Bearer YourAccessToken",
		}),
	)
	if err != nil {
		return nil, err
	}

	// サービスのリソース情報を設定する。
	res, err := resource.New(context.Background(),
		resource.WithAttributes(
			semconv.ServiceNameKey.String(serviceName),
			semconv.ServiceVersionKey.String("v1"),
			attribute.String("environment", "local"),
		),
	)
	if err != nil {
		return nil, err
	}

	// トレーサープロバイダを作成し、OTLPエクスポーターとリソース情報を設定する。
	return sdktrace.NewTracerProvider(
		sdktrace.WithBatcher(exp),
		sdktrace.WithResource(res),
	), nil
}
