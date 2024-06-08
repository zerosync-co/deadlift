use crate::{schema::*, services::db};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = workflows)]
pub struct Workflow {
    id: i32,
    name: String,
    description: Option<String>,
    pipeline: String,
}

impl Workflow {
    pub fn create(name: String, description: Option<String>, pipeline: String) -> QueryResult<i32> {
        let values = (
            workflows::name.eq(name),
            workflows::description.eq(description),
            workflows::pipeline.eq(pipeline),
        );

        let conn = &mut db::connection()?;
        let created_workflow = diesel::insert_into(workflows::table)
            .values(values)
            .get_result::<Self>(conn)?;

        Ok(created_workflow.id)
    }

    pub fn find_by_id(id: i32) -> QueryResult<Self> {
        let conn = &mut db::connection()?;
        workflows::table.find(id).first(conn)
    }

    pub fn get_pipeline(&self) -> impl Iterator<Item = &str> {
        self.pipeline.split(',')
    }
}
