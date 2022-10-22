use crate::model::record::Record;
use crate::schema::records::dsl::{self, records as table};
use diesel::result::Error;
use diesel::{prelude::*, select};
use nanoid::nanoid;
use std::time::SystemTime;

pub fn get_all(conn: &mut PgConnection) -> Option<Vec<Record>> {
    let res = table.get_results::<Record>(conn);
    match res {
        Ok(r) => Some(r),
        Err(_) => None,
    }
}

pub fn get_by_id(conn: &mut PgConnection, id: &str) -> Option<Record> {
    let res = table
        .filter(dsl::id.eq(id))
        .limit(1)
        .get_result::<Record>(conn);
    match res {
        Ok(r) => Some(r),
        Err(_) => None,
    }
}

pub fn save(conn: &mut PgConnection, url: &str, token: &str) -> Option<Record> {
    let now = select(diesel::dsl::now)
        .get_result::<SystemTime>(conn)
        .unwrap();
    let record = Record {
        id: nanoid!(10),
        url: String::from(url),
        token: Some(String::from(token)),
        expired_at: now,
        created_at: now,
        updated_at: now,
    };
    let res = diesel::insert_into(table)
        .values(&record)
        .get_result::<Record>(conn);
    match res {
        Ok(r) => Some(r),
        Err(_) => None,
    }
}

pub fn delete(conn: &mut PgConnection, id: &str, token: &str) -> Result<usize, Error> {
    diesel::delete(table.filter(dsl::id.eq(id)).filter(dsl::token.eq(token))).execute(conn)
}
