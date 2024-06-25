CREATE TABLE workflow_modules (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    workflow_id INTEGER NOT NULL,
    module_hash TEXT NOT NULL,
    parent_workflow_module_id INTEGER,
    FOREIGN KEY (workflow_id) REFERENCES workflows(id),
    FOREIGN KEY (module_hash) REFERENCES modules(hash)
);
