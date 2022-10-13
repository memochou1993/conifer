use crate::database::schema::records::dsl::{id, records as table};
use crate::model::record::Record;
use diesel::prelude::*;
use diesel::result::Error;
use nanoid::nanoid;

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
    let record = Record {
        id: nanoid!(10),
        url: String::from(_url),
    };

    diesel::insert_into(table)
        .values(&record)
        .get_result::<Record>(conn)
        .optional()
}
