CREATE TABLE customer_type_discounts
(
    customer_type INTEGER PRIMARY KEY,
    discount INTEGER NOT NULL
);

insert into customer_type_discounts
values
       (1, 0),
       (2, 50),
       (3, 30);