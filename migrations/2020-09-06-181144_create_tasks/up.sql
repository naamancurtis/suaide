CREATE TABLE IF NOT EXISTS suaide (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
   	ticket TEXT,
	description TEXT NOT NULL,
	opened BigInt NOT NULL,
	closed BigInt
);
