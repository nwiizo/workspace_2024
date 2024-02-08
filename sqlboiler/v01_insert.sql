-- 著者テーブルにサンプルデータを挿入
INSERT INTO authors (name) VALUES ('Sample Author 1');
INSERT INTO authors (name) VALUES ('Sample Author 2');
INSERT INTO authors (name) VALUES ('Sample Author 3');

-- 出版社テーブルにサンプルデータを挿入
INSERT INTO publishers (name) VALUES ('Sample Publisher 1');
INSERT INTO publishers (name) VALUES ('Sample Publisher 2');
INSERT INTO publishers (name) VALUES ('Sample Publisher 3');

-- 利用者テーブルにサンプルデータを挿入
INSERT INTO users (family_name, given_name, email_address, registration_date) VALUES ('Yamada', 'Taro', 'taro@example.com', '2021-01-01');
INSERT INTO users (family_name, given_name, email_address, registration_date) VALUES ('Suzuki', 'Hanako', 'hanako@example.com', '2021-02-01');
INSERT INTO users (family_name, given_name, email_address, registration_date) VALUES ('Tanaka', 'Ichiro', 'ichiro@example.com', '2021-03-01');

-- 書籍テーブルにサンプルデータを挿入
INSERT INTO books (title, author_id, publisher_id, isbn, year_published) VALUES ('Sample Book 1', 1, 1, '1234567890', 2021);
INSERT INTO books (title, author_id, publisher_id, isbn, year_published) VALUES ('Sample Book 2', 2, 2, '0987654321', 2020);
INSERT INTO books (title, author_id, publisher_id, isbn, year_published) VALUES ('Sample Book 3', 3, 3, '1122334455', 2022);

-- 貸出記録テーブルにサンプルデータを挿入
INSERT INTO loans (book_id, user_id, loan_date, return_date) VALUES (1, 1, '2022-01-01', '2022-01-15');
INSERT INTO loans (book_id, user_id, loan_date, return_date) VALUES (2, 2, '2022-01-05', '2022-01-20');
INSERT INTO loans (book_id, user_id, loan_date) VALUES (3, 3, '2022-01-10');
