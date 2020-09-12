CREATE TABLE IF NOT EXISTS suaide (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
   	ticket TEXT UNIQUE,
	description TEXT NOT NULL,
	status SmallInt NOT NULL,
	opened BigInt NOT NULL,
	closed BigInt
);
