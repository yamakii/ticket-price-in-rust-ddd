table! {
    customer_type_discounts (customer_type) {
        customer_type -> Int4,
        disconnect -> Int4,
    }
}

table! {
    movies (id) {
        id -> Int4,
        title -> Text,
        base_price -> Int4,
    }
}

table! {
    order_details (order_id, no) {
        order_id -> Int4,
        no -> Int4,
        customer_type -> Int4,
        ticket_count -> Int4,
        ticket_price -> Int4,
    }
}

table! {
    orders (id) {
        id -> Int4,
        movie_id -> Int4,
        start_at -> Date,
    }
}

allow_tables_to_appear_in_same_query!(
    customer_type_discounts,
    movies,
    order_details,
    orders,
);
