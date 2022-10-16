use crate::model::record::Record;
use crate::schema::records::dsl::{id, records as table};
use diesel::result::Error;
use diesel::{prelude::*, select};
use nanoid::nanoid;
use std::time::SystemTime;

pub fn get_all(conn: &mut PgConnection) -> Result<Option<Vec<Record>>, Error> {
    table.get_results::<Record>(conn).optional()
}

pub fn get_by_id(conn: &mut PgConnection, _id: &str) -> Result<Option<Record>, Error> {
    table
        .filter(id.eq(_id))
        .limit(1)
        .get_result::<Record>(conn)
        .optional()
}

pub fn save(conn: &mut PgConnection, _url: &str) -> Result<Option<Record>, Error> {
    let now = select(diesel::dsl::now).get_result::<SystemTime>(conn)?;
    let record = Record {
        id: nanoid!(10),
        url: String::from(_url),
        expired_at: now,
        updated_at: now,
        created_at: now,
    };

    diesel::insert_into(table)
        .values(&record)
        .get_result::<Record>(conn)
        .optional()
}

pub fn delete(conn: &mut PgConnection, _id: &str) -> Result<usize, Error> {
    diesel::delete(table.filter(id.eq(_id))).execute(conn)
}
