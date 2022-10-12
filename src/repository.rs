use crate::model::Record;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn connect() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_record(conn: &mut PgConnection, _id: &str) -> Record {
    use crate::schema::records::dsl::{id, records};
    records
        .filter(id.eq(_id))
        .limit(1)
        .get_result::<Record>(conn)
        .expect("Error loading record")
}

pub fn get_records(conn: &mut PgConnection) -> Vec<Record> {
    use crate::schema::records::dsl::records;
    records
        .get_results::<Record>(conn)
        .expect("Error loading records")
}
