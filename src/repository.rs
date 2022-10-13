use crate::model::Record;
use crate::schema::records::dsl::{id, records};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenvy::dotenv;
use nanoid::nanoid;
use std::env;

pub fn connect() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_records(conn: &mut PgConnection) -> Result<Option<Vec<Record>>, Error> {
    records.get_results::<Record>(conn).optional()
}

pub fn get_record(conn: &mut PgConnection, _id: &str) -> Result<Option<Record>, Error> {
    records
        .filter(id.eq(_id))
        .limit(1)
        .get_result::<Record>(conn)
        .optional()
}

pub fn store_record(conn: &mut PgConnection, _url: &str) -> Result<Option<Record>, Error> {
    use crate::schema::records;
    let record = Record {
        id: nanoid!(10),
        url: String::from(_url),
    };

    diesel::insert_into(records::table)
        .values(&record)
        .get_result::<Record>(conn)
        .optional()
}
