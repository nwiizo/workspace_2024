-- 貸出記録テーブルのデータを削除
DELETE FROM loans;

-- 書籍テーブルのデータを削除
DELETE FROM books;

-- 利用者テーブルのデータを削除
DELETE FROM users;

-- 出版社テーブルのデータを削除
DELETE FROM publishers;

-- 著者テーブルのデータを削除
DELETE FROM authors;

-- シーケンスをリセット（シリアルキーを使用している場合）
ALTER SEQUENCE authors_author_id_seq RESTART WITH 1;
ALTER SEQUENCE publishers_publisher_id_seq RESTART WITH 1;
ALTER SEQUENCE users_user_id_seq RESTART WITH 1;
ALTER SEQUENCE books_book_id_seq RESTART WITH 1;
ALTER SEQUENCE loans_loan_id_seq RESTART WITH 1;
