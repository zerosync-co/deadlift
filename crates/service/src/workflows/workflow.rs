use crate::{schema::*, services::db};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::module::WorkflowModule;

#[derive(Serialize, Deserialize)]
pub struct Workflow {
    id: i32,
    name: String,
    description: Option<String>,
    pipeline: Vec<WorkflowModule>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = workflows)]
pub struct WorkflowRecord {
    id: i32,
    name: String,
    description: Option<String>,
}

impl Workflow {
    pub fn create(
        name: String,
        description: Option<String>,
        pipeline: Vec<String>,
    ) -> QueryResult<i32> {
        let connection = &mut db::connection()?;

        connection.transaction(|conn| {
            let workflow_values = (
                workflows::name.eq(name),
                workflows::description.eq(description),
            );

            let created_workflow_record = diesel::insert_into(workflows::table)
                .values(workflow_values)
                .get_result::<WorkflowRecord>(conn)?;

            WorkflowModule::create(pipeline, created_workflow_record.id, conn)?;

            Ok(created_workflow_record.id)
        })
    }

    pub fn find_by_id(id: i32) -> QueryResult<Self> {
        let conn = &mut db::connection()?;
        let workflow_record = workflows::table.find(id).first::<WorkflowRecord>(conn)?;

        let pipeline = WorkflowModule::get_pipeline_from_workflow_id(id)?;

        Ok(Self {
            id: workflow_record.id,
            name: workflow_record.name,
            description: workflow_record.description,
            pipeline,
        })
    }

    pub fn get_pipeline(&self) -> impl Iterator<Item = &WorkflowModule> {
        self.pipeline.iter()
    }
}
