CREATE TABLE IF NOT EXISTS workflow_modules (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    workflow_id INTEGER NOT NULL,
    module_hash TEXT NOT NULL,
    parent_workflow_module_id INTEGER,
    FOREIGN KEY (workflow_id) REFERENCES workflows(id) ON DELETE CASCADE, --FIXME-- delete cascade does not work !?
    FOREIGN KEY (module_hash) REFERENCES modules(hash)
);
