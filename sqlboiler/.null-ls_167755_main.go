package main

import (
	"context"
	"database/sql"
	"log"

	_ "github.com/lib/pq"
	"github.com/nwiizo/workspace_2024/sqlboiler/models" // 生成されたモデルのインポート
)

func main() {
	db, err := sql.Open("postgres", "postgres://user:pass@localhost/dbname?sslmode=disable")
	if err != nil {
		log.Fatal(err)
	}

	// 全ての書籍を取得
	books, err := models.Books().All(context.Background(), db)
	if err != nil {
		log.Fatal(err)
	}

	for _, book := range books {
		log.Printf("Book: %+v\n", book)
	}
}
