use crate::{schema::*, services::db};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = modules)]
pub struct Module {
    hash: String,
    binary: Vec<u8>,
    title: String, // REVIEW-- drop this field? it seems unnecessary given the subject field-- change to name and change subject to type; add (name, type) unique constraint
    description: Option<String>,
    subject: String,
}

impl Module {
    pub fn create(
        binary: Vec<u8>,
        title: String,
        description: Option<String>,
        subject: String,
    ) -> QueryResult<String> {
        let digest = Sha256::digest(&binary);
        let hash = hex::encode(digest);

        let values = (
            modules::hash.eq(hash.to_string()),
            modules::binary.eq(binary),
            modules::title.eq(title),
            modules::description.eq(description),
            modules::subject.eq(subject),
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

    pub fn get_hash_by_subject(subject: &str) -> QueryResult<String> {
        let conn = &mut db::connection()?;
        modules::table
            .filter(modules::subject.eq(subject))
            .select(modules::hash)
            .first(conn)
    }

    pub fn list() -> QueryResult<Vec<serde_json::Value>> {
        let conn = &mut db::connection()?;
        let result: Vec<(String, String, Option<String>, String)> = modules::table
            .select((
                modules::hash,
                modules::title,
                modules::description,
                modules::subject,
            ))
            .load(conn)?;

        Ok(result
            .into_iter()
            .map(|(hash, title, description, subject)| {
                serde_json::json!({
                    "hash": hash,
                    "title": title,
                    "description": description,
                    "subject": subject
                })
            })
            .collect::<Vec<serde_json::Value>>())
    }
}
