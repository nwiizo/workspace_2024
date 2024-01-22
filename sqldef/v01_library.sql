-- 著者テーブル
create table authors (
  author_id serial primary key,
  name varchar(100) not null
);

-- 出版社テーブル
create table publishers (
  publisher_id serial primary key,
  name varchar(100) not null
);

-- 利用者テーブル
create table users (
  user_id serial primary key,
  user_name varchar(100) not null,
  email_address varchar(100),
  registration_date date not null
);

-- 書籍テーブル
create table books (
  book_id serial primary key,
  title varchar(255) not null,
  author_id integer not null,
  publisher_id integer not null,
  isbn varchar(20),
  year_published integer
);

-- 貸出記録テーブル
create table loans (
  loan_id serial primary key,
  book_id integer not null,
  user_id integer not null,
  loan_date date not null,
  return_date date
);

-- 外部キー制約の追加
alter table books add constraint fk_books_author_id foreign key (author_id) references authors(author_id);
alter table books add constraint fk_books_publisher_id foreign key (publisher_id) references publishers(publisher_id);
alter table loans add constraint fk_loans_book_id foreign key (book_id) references books(book_id);
alter table loans add constraint fk_loans_user_id foreign key (user_id) references users(user_id);
