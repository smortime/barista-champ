DROP TABLE IF EXISTS orders;
DROP TABLE IF EXISTS customers;
DROP TABLE IF EXISTS coffees;
DROP TABLE IF EXISTS regions;
DROP TABLE IF EXISTS roasters;
DROP TABLE IF EXISTS drink_types;

CREATE TABLE drink_types (
	drink_type TEXT NOT NULL
);

CREATE TABLE roasters (
	id TEXT PRIMARY KEY,
	name TEXT NOT NULL
);


CREATE TABLE regions (
	id TEXT PRIMARY KEY,
	name TEXT NOT NULL
);


CREATE TABLE coffees (
	id TEXT PRIMARY KEY,
	name TEXT,
	tasting_notes BLOB,
	style TEXT NOT NULL,
	region_id TEXT NOT NULL,
	roaster_id TEXT NOT NULL,
	FOREIGN KEY (region_id)
		REFERENCES regions (id)
			ON DELETE CASCADE
			ON UPDATE NO ACTION,
	FOREIGN KEY (roaster_id)
		REFERENCES roasters (id)
			ON DELETE CASCADE
			ON UPDATE NO ACTION
);

CREATE TABLE customers (
	id TEXT PRIMARY KEY,
	name TEXT NOT NULL
);


CREATE TABLE orders (
	id TEXT PRIMARY KEY,
	coffee_id TEXT,
	drink_type TEXT,
	customer_id TEXT,
	FOREIGN KEY (coffee_id)
		REFERENCES coffees (id)
			ON DELETE CASCADE
			ON UPDATE NO ACTION,
	FOREIGN KEY (customer_id)
		REFERENCES customers (id)
			ON DELETE CASCADE
			ON UPDATE NO ACTION
);

