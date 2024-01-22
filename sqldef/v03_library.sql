-- ユーザーテーブルの作成
create table users (
  user_id serial primary key,
  family_name varchar(100) not null,
  given_name varchar(100) not null,
  email_address varchar(254) not null,
  registration_date date not null
);

-- メールアドレスに対するユニークキー制約（ユニークインデックス）
create unique index idx_users_email_address on users(email_address);
