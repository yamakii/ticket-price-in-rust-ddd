CREATE TABLE orders
(
    id       SERIAL PRIMARY KEY,
    movie_id INTEGER NOT NULL,
    start_at DATE    NOT NULL
);

CREATE TABLE order_details
(
    order_id      INTEGER NOT NULL,
    no            INTEGER NOT NULL,
    customer_type INTEGER NOT NULL,
    ticket_count  INTEGER NOT NULL,
    ticket_price  INTEGER NOT NULL,
    PRIMARY KEY (order_id, no)
);
