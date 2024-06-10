use crate::{schema::*, services::db};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use wasmer_cache::Hash;

#[derive(Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = modules)]
pub struct Module {
    hash: String,
    binary: Vec<u8>,
    title: String,
    description: Option<String>,
}

impl Module {
    pub fn create(
        binary: Vec<u8>,
        title: String,
        description: Option<String>,
    ) -> QueryResult<String> {
        let hash = Hash::generate(&binary);

        let values = (
            modules::hash.eq(hash.to_string()),
            modules::binary.eq(binary),
            modules::title.eq(title),
            modules::description.eq(description),
        );

        let conn = &mut db::connection()?;
        diesel::insert_into(modules::table)
            .values(values)
            .execute(conn)?;

        Ok(hash.to_string())
    }

    pub fn get_binary_by_hash(hash: &str) -> QueryResult<Vec<u8>> {
        let conn = &mut db::connection()?;
        modules::table
            .find(hash)
            .select(modules::binary)
            .first(conn)
    }
}
