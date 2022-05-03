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
	tasting_notes BLOB,
	style TEXT,
	region_id TEXT,
	roaster_id TEXT,
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
	drink_type_id TEXT,
	customer_id TEXT,
	FOREIGN KEY (coffee_id)
		REFERENCES coffees (id)
			ON DELETE CASCADE
			ON UPDATE NO ACTION,
	FOREIGN KEY (drink_type_id)
		REFERENCES drink_types (rowid)
			ON DELETE CASCADE
			ON UPDATE NO ACTION,
	FOREIGN KEY (customer_id)
		REFERENCES customers (id)
			ON DELETE CASCADE
			ON UPDATE NO ACTION
);

