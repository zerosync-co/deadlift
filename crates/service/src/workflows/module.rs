use crate::{
    modules::module::Module,
    schema::*,
    services::db::{self, DbConnection},
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = workflow_modules)]
pub struct WorkflowModule {
    id: i32,
    workflow_id: i32,
    module_hash: String,
    parent_workflow_module_id: Option<i32>,
}

impl WorkflowModule {
    pub fn create(
        pipeline: Vec<String>,
        workflow_id: i32,
        conn: &mut DbConnection,
    ) -> QueryResult<Vec<Self>> {
        let mut created_worflow_modules = Vec::new();

        let mut current_workflow_module_id = None;
        for subject in pipeline {
            let hash = Module::get_hash_by_subject(&subject)?;

            let workflow_module_values = (
                workflow_modules::workflow_id.eq(workflow_id),
                workflow_modules::module_hash.eq(hash),
                workflow_modules::parent_workflow_module_id.eq(current_workflow_module_id),
            );

            let workflow_module = diesel::insert_into(workflow_modules::table)
                .values(workflow_module_values)
                .get_result::<WorkflowModule>(conn)?;

            current_workflow_module_id = Some(workflow_module.id);

            created_worflow_modules.push(workflow_module)
        }

        Ok(created_worflow_modules)
    }

    pub fn get_pipeline_from_workflow_id(workflow_id: i32) -> QueryResult<Vec<Self>> {
        let connection = &mut db::connection()?;

        let first_workflow_module = workflow_modules::table
            .filter(
                workflow_modules::workflow_id
                    .eq(workflow_id)
                    .and(workflow_modules::parent_workflow_module_id.is_null()),
            )
            .first::<Self>(connection)?;

        let mut pipeline = vec![];

        let mut next_workflow_module = Some(first_workflow_module);
        while let Some(workflow_module) = next_workflow_module {
            let workflow_module_id = workflow_module.id;
            pipeline.push(workflow_module);

            next_workflow_module =
                workflow_modules::table
                    .filter(workflow_modules::workflow_id.eq(workflow_id).and(
                        workflow_modules::parent_workflow_module_id.eq(Some(workflow_module_id)),
                    ))
                    .first::<Self>(connection)
                    .optional()?;
        }

        Ok(pipeline)
    }

    pub fn get_hash(&self) -> &str {
        &self.module_hash
    }
}
