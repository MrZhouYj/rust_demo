use bigdecimal::BigDecimal;

#[derive(Queryable, Debug)]
pub struct Order {
    pub id: i32,
    pub member_id: i32,
    pub market_id: String,
    pub ask_or_bid: String,
    pub state: i32,
    pub price: BigDecimal,
    pub volume: BigDecimal,
}
