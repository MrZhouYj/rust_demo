use crate::db_connection::db_connection;
use crate::models::Order;
use crate::schema::orders;
use crate::schema::orders::dsl::orders as orders_dsl;
use actix_web::web;
use diesel::prelude::*;

pub struct Matcher {
    pub ask_orders: Vec<Order>,
    pub bid_orders: Vec<Order>,
}

impl Matcher {
    pub fn default() -> Matcher {
        let conn = db_connection();
        let ask_orders = orders_dsl
            .filter(
                orders::ask_or_bid
                    .eq("ask")
                    .and(orders::state.eq(100))
                    .and(orders::market_id.eq("btcusdt")),
            )
            .order(orders::id.asc())
            .order(orders::price.asc())
            .load::<Order>(&conn)
            .unwrap();

        let bid_orders = orders_dsl
            .filter(
                orders::ask_or_bid
                    .eq("bid")
                    .and(orders::state.eq(100))
                    .and(orders::market_id.eq("btcusdt")),
            )
            .order(orders::id.asc())
            .order(orders::price.desc())
            .load::<Order>(&conn)
            .unwrap();

        Matcher {
            ask_orders: ask_orders,
            bid_orders: bid_orders,
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.ask_orders.push(order);
        Matcher::do_matcher();
    }
    fn do_matcher() {}
}
