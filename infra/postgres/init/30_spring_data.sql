CREATE TABLE book
(
  id          SERIAL PRIMARY KEY,
  isbn        VARCHAR(100),
  title       VARCHAR(100),
  price       INT,
  publisher   VARCHAR(100),
  published   TIMESTAMP,
  attach      VARCHAR(100)
);

CREATE TABLE app_user
(
  id          SERIAL PRIMARY KEY,
  username    VARCHAR(100),
  password    VARCHAR(200),
  enabled     INT
);

CREATE TABLE review
(
  id          SERIAL PRIMARY KEY,
  name        VARCHAR(100),
  body        VARCHAR(200),
  book_id     INT
);


INSERT INTO book(id, isbn, title, price, publisher, published, attach)
   VALUES(1, '978-4-7981-6364-2','独習Python', 3000, '翔泳社', '2020-06-22', 'dl');
INSERT INTO book(id, isbn, title, price, publisher, published, attach) 
  VALUES(2, '978-4-7981-6751-0','Spring Boot入門', 1600, '翔泳社', '2020-05-31', 'cd');
INSERT INTO book(id, isbn, title, price, publisher, published, attach)
  VALUES(3, '978-4-8026-1252-4','Slim Webアプリケーション開発', 3600, 'ソシム', '2020-05-25', 'dvd');
INSERT INTO book(id, isbn, title, price, publisher, published, attach)
  VALUES(4, '978-4-8222-5391-2','iPhoneアプリ超入門', 2200, '日経BP', '2020-02-28', NULL);
INSERT INTO book(id, isbn, title, price, publisher, published, attach)
  VALUES(5, '978-4-8222-8653-8','基礎からしっかり学ぶC#の教科書', 2900, '日経BP', '2019-12-20', 'dl');
INSERT INTO book(id, isbn, title, price, publisher, published, attach)
  VALUES(6, '978-4-7981-6365-9','独習ASP.NET Webフォーム', 3800, '翔泳社', '2020-02-17', 'cd');
INSERT INTO book(id, isbn, title, price, publisher, published, attach)
  VALUES(7, '978-4-8026-1226-5','SQLデータ分析・活用入門', 2600, 'ソシム', '2019-09-12', 'dvd');
INSERT INTO book(id, isbn, title, price, publisher, published, attach)
  VALUES(8, '978-4-8156-0182-9','これからはじめるVue.js実践入門', 3380, 'SBクリエイティブ', '2019-08-22', 'dl');
INSERT INTO book(id, isbn, title, price, publisher, published, attach)
  VALUES(9, '978-4-7980-5759-0','はじめてのAndroidアプリ開発', 3200, '秀和システム', '2019-08-10', 'cd');
INSERT INTO book(id, isbn, title, price, publisher, published, attach)
  VALUES(10, '978-4-7981-5112-0', '独習Java 新版' , 2980, '翔泳社', '2019-05-15', 'dl');

/*
INSERT INTO book(id, isbn, title, price, publisher, published, attach)
  VALUES(10, '978-4-7981-5112-0', NULL , 2980, '翔泳社', '2019-05-15', 'dl');
*/
  
INSERT INTO app_user(id, username, password, enabled)
  VALUES(1, 'yamada', '$2a$10$/Q4O4F7ELhz2V5ZpeodUh.UbUyrvUkSRe8m/JzDVjqJQPuYODU2OO', 1);
INSERT INTO app_user(id, username, password, enabled)
  VALUES(2, 'suzuki', '$2a$10$/Q4O4F7ELhz2V5ZpeodUh.UbUyrvUkSRe8m/JzDVjqJQPuYODU2OO', 1);
INSERT INTO app_user(id, username, password, enabled)
  VALUES(3, 'tanaka', '$2a$10$/Q4O4F7ELhz2V5ZpeodUh.UbUyrvUkSRe8m/JzDVjqJQPuYODU2OO', 1);
  
INSERT INTO review(id, name, body, book_id) VALUES(1, '山田太郎', '丁寧な説明で分かりやすい。', 1);
INSERT INTO review(id, name, body, book_id) VALUES(2, '鈴木次郎', '初めての人におすすめの入門書だった。', 1);
INSERT INTO review(id, name, body, book_id) VALUES(3, '田中三郎', 'まさに、独学するのにうってつけだと思う。', 1);
INSERT INTO review(id, name, body, book_id) VALUES(4, '山田太郎', 'コンパクトにまとまっていて良い。', 2);
