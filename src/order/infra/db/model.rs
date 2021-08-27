use chrono::NaiveDate;

#[derive(Queryable)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub basic_price: i32,
}

#[derive(Queryable)]
pub struct CustomerTypeDiscounts {
    pub customer_type: i32,
    pub disconnect: i32,
}

#[derive(Queryable)]
pub struct Orders {
    pub id: i32,
    pub movie_id: i32,
    pub start_at: NaiveDate,
}

#[derive(Queryable)]
pub struct OrderDetails {
    pub order_id: i32,
    pub no: i32,
    pub customer_type: i32,
    pub ticket_count: i32,
    pub ticket_price: i32,
}
