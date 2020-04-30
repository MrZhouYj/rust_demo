table! {
    orders (id) {
      id -> Integer,
      member_id -> Integer,
      market_id -> Varchar,
      ask_or_bid -> Varchar,
      state -> Integer,
      price -> Decimal,
      volume -> Decimal,
    }
}
