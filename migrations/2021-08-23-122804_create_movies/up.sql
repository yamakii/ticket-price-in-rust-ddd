CREATE TABLE movies
(
    id         SERIAL PRIMARY KEY,
    title      TEXT    NOT NULL,
    base_price INTEGER NOT NULL
);

insert into movies (title, base_price)
values
    ('ステキな恋愛映画', 1000),
    ('ものすごいSF映画', 1800),
    ('おもしろいアニメ', 1200);
