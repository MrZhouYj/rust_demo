#[derive(Queryable, Debug)]
pub struct Orders {
    pub id: u64,
    pub member_id: u32,
    pub market_id: String,
    pub ask_or_bid: String
}
