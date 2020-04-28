use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, MysqlConnection};

fn get_db_url() -> String {
    dotenv::dotenv().ok();
    std::env::var("DATABASE_URL")
    .expect("DATABASE_URL not found in .env file!")
}

pub fn db_pool() -> Pool<ConnectionManager<MysqlConnection>> {
  let database_url = get_db_url();
  r2d2::Pool::builder()
    .build(ConnectionManager::<MysqlConnection>::new(&database_url))
    .unwrap()
}


pub fn db_connection() -> MysqlConnection {
  let database_url = get_db_url();
  MysqlConnection::establish(&database_url).unwrap()
}
