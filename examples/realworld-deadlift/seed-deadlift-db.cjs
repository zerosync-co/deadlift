const sqlite3 = require('sqlite3').verbose();
const fs = require('fs');
const crypto = require('crypto');

let db = new sqlite3.Database('../../crates/service/database.sqlite', (err) => {
	if (err) {
		console.error(err.message);
		return;
	}
	console.log('Connected to the SQLite database.');
});

const createTableSql = `
CREATE TABLE IF NOT EXISTS modules (
    hash TEXT PRIMARY KEY NOT NULL,
    binary BLOB NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    subject TEXT NOT NULL UNIQUE
);`;

function insertModule(moduleData) {
	const { wasmPath, title, description, subject } = moduleData;
	const binary = fs.readFileSync(wasmPath);
	const hash = crypto.createHash('sha256').update(binary).digest('hex');

	const insertSql = `INSERT INTO modules (hash, binary, title, description, subject) VALUES (?, ?, ?, ?, ?)`;
	db.run(insertSql, [hash, binary, title, description, subject], (err) => {
		if (err) {
			console.error(err.message);
		} else {
			console.log(`Inserted module with hash: ${hash}`);
		}
	});
}

db.serialize(() => {
	db.run(createTableSql, (err) => {
		if (err) {
			console.error(err.message);
		}
	});

	const modules = [
		{
			wasmPath: './wasm-fetch/pkg/wasm_fetch_bg.wasm',
			title: 'Receive blog data',
			description: null,
			subject: 'deadlift.modules.ingest.realworld-fetch'
		},
		{
			wasmPath: './wasm-fetch-filter/target/wasm32-unknown-unknown/release/wasm_fetch_filter.wasm',
			title: 'Filter for article creation',
			description: null,
			subject: 'deadlift.modules.default.realworld-fetch-filter'
		},
		{
			wasmPath:
				'./wasm-send-email-notification/target/wasm32-unknown-unknown/release/wasm_send_email_notification.wasm',
			title: 'Send email notification',
			description: null,
			subject: 'deadlift.modules.deliver.realworld-send-email-notification'
		}
	];

	modules.forEach(insertModule);
});

db.close((err) => {
	if (err) {
		console.error(err.message);
	}
	console.log('Closed the database connection.');
});
