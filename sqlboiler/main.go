package main

import (
	"context"
	"database/sql"
	"fmt"
	"log"

	_ "github.com/lib/pq"
	"github.com/nwiizo/workspace_2024/sqlboiler/models" // 生成されたモデルのインポート
	"github.com/volatiletech/null/v8"
	"github.com/volatiletech/sqlboiler/v4/boil"
	"github.com/volatiletech/sqlboiler/v4/queries/qm"
)

func main() {
	// データベース接続
	db, err := sql.Open(
		"postgres",
		"postgres://postgres:postgres@localhost/postgres?sslmode=disable",
	)
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	ctx := context.Background()

	// SELECT: 全ての書籍を取得
	allBooks, err := models.Books().All(ctx, db)
	if err != nil {
		log.Fatal(err)
	}
	for _, book := range allBooks {
		log.Printf("Book: %+v\n", book)
	}

	fmt.Println("Select: 高度なクエリでの書籍の取得")
	books, err := models.Books(
		models.BookWhere.Title.EQ("Specific Title"),
		models.BookWhere.AuthorID.EQ(1),
		qm.Limit(10),
	).All(ctx, db)
	if err != nil {
		log.Fatal(err)
	}
	for _, book := range books {
		fmt.Println("Book:", book.Title)
	}

	fmt.Println("Count: 書籍の数を数える")
	count, err := models.Books(models.BookWhere.Title.EQ("Specific Title")).Count(ctx, db)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Count:", count)

	fmt.Println("Exists: 特定の条件に一致する書籍が存在するかを確認")
	exists, err := models.Books(models.BookWhere.Title.EQ("Specific Title")).Exists(ctx, db)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Exists:", exists)

	fmt.Println("Insert: 書籍の挿入")
	newBook := &models.Book{
		Title:         "New Book",
		AuthorID:      1,
		PublisherID:   1,
		Isbn:          null.StringFrom("1234567890"),
		YearPublished: null.IntFrom(2023),
	}
	err = newBook.Insert(ctx, db, boil.Infer())
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Update: 書籍の更新")
	newBook.Title = "Updated Title"
	_, err = newBook.Update(ctx, db, boil.Infer())
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Upsert: 書籍のアップサート")
	upsertBook := &models.Book{
		BookID:        newBook.BookID,
		Title:         "Upserted Title",
		AuthorID:      2,
		PublisherID:   2,
		Isbn:          null.StringFrom("0987654321"),
		YearPublished: null.IntFrom(2024),
	}
	err = upsertBook.Upsert(ctx, db, true, []string{"book_id"}, boil.Infer(), boil.Infer())
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Delete: 書籍の削除")
	_, err = newBook.Delete(ctx, db)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Reload: 書籍の再読み込み")
	err = newBook.Reload(ctx, db)
	if err != nil {
		if err == sql.ErrNoRows {
			fmt.Println("Reload: 書籍が見つかりませんでした")
		} else {
			log.Fatal(err)
		}
	}
	// Eager Loading の例
	// ユーザーと関連する書籍を取得
	// user, err := models.FindUser(ctx, db, 1, qm.Load("Books"))
	// if err != nil {
	//     log.Fatal(err)
	// }
	// for _, book := range user.R.Books {
	//     fmt.Println("Book:", book.Title)
	// }

	// デバッグ出力の例
	// boil.DebugMode = true
	// books, _ = models.Books().All(ctx, db)
	// boil.DebugMode = false

	// Raw Query の例
	// _, err = queries.Raw("SELECT * FROM books WHERE title = 'New Book'").QueryAll(ctx, db)
	// if err != nil {
	//     log.Fatal(err)
	// }

	// Hook の例
	// func myBookHook(ctx context.Context, exec boil.ContextExecutor, book *models.Book) error {
	//     fmt.Println("Book Hook Triggered")
	//     return nil
	// }
	// models.AddBookHook(boil.BeforeInsertHook, myBookHook)

	// null パッケージの使用例
	// newBook.Isbn = null.StringFromPtr(nil) // ISBN を null に設定
}
