package main

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"time"

	"github.com/gofiber/fiber/v3"
	_ "github.com/lib/pq"
)

const (
	host     = "db"
	port     = 5432
	user     = "postgres"
	password = "password"
	dbname   = "mydb"
)

// Connect to the database
func Connect() (*sql.DB, error) {
	psqlInfo := fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode=disable", host, port, user, password, dbname)
	db, err := sql.Open("postgres", psqlInfo)
	if err != nil {
		return nil, err
	}
	return db, nil
}

// User model
type User struct {
	ID        int       `json:"id"`
	Name      string    `json:"name"`
	Email     string    `json:"email"`
	Password  string    `json:"password"`
	CreatedAt time.Time `json:"created_at"`
}

// Post model
type Post struct {
	ID        int       `json:"id"`
	UserID    int       `json:"user_id"`
	Title     string    `json:"title"`
	Content   string    `json:"content"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}

func main() {
	app := fiber.New()

	// データベース接続
	db, err := Connect()
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	// Create User
	app.Post("/users", func(c fiber.Ctx) error {
		user := new(User)
		if err := json.Unmarshal(c.Body(), user); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
				"error": "Invalid request body",
			})
		}

		// パスワードのハッシュ化やバリデーションを行うことが推奨される
		// 簡易実装のため、ここでは省略

		// データベースにユーザーを作成
		_, err := db.Exec("INSERT INTO users (name, email, password) VALUES ($1, $2, $3)", user.Name, user.Email, user.Password)
		if err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
				"error": "Failed to create user",
			})
		}

		return c.Status(fiber.StatusCreated).JSON(fiber.Map{
			"message": "User created",
		})
	})

	// Get User
	app.Get("/users/:id", func(c fiber.Ctx) error {
		id := c.Params("id")

		// データベースからユーザーを取得
		row := db.QueryRow("SELECT * FROM users WHERE id = $1", id)
		user := new(User)
		if err := row.Scan(&user.ID, &user.Name, &user.Email, &user.Password, &user.CreatedAt); err != nil {
			if err == sql.ErrNoRows {
				return c.Status(fiber.StatusNotFound).JSON(fiber.Map{
					"error": "User not found",
				})
			}
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
				"error": "Failed to get user",
			})
		}

		return c.JSON(user)
	})

	// Create Post
	app.Post("/posts", func(c fiber.Ctx) error {
		post := new(Post)
		if err := json.Unmarshal(c.Body(), post); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
				"error": "Invalid request body",
			})
		}

		// データベースに記事を作成
		_, err := db.Exec("INSERT INTO posts (user_id, title, content) VALUES ($1, $2, $3)", post.UserID, post.Title, post.Content)
		if err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
				"error": "Failed to create post",
			})
		}

		return c.Status(fiber.StatusCreated).JSON(fiber.Map{
			"message": "Post created",
		})
	})

	// Get Post
	app.Get("/posts/:id", func(c fiber.Ctx) error {
		id := c.Params("id")

		// データベースから記事を取得
		row := db.QueryRow("SELECT * FROM posts WHERE id = $1", id)
		post := new(Post)
		if err := row.Scan(&post.ID, &post.UserID, &post.Title, &post.Content, &post.CreatedAt, &post.UpdatedAt); err != nil {
			if err == sql.ErrNoRows {
				return c.Status(fiber.StatusNotFound).JSON(fiber.Map{
					"error": "Post not found",
				})
			}
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
				"error": "Failed to get post",
			})
		}

		return c.JSON(post)
	})

	log.Fatal(app.Listen(":3000"))
}
