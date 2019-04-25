-- Your SQL goes here
CREATE TABLE number_type(
	   number_type_id SERIAL PRIMARY KEY,
	   number_type_name TEXT NOT NULL
);

CREATE TABLE number_status(
	   number_status_id SERIAL PRIMARY KEY,
	   number_status_name TEXT NOT NULL
);

CREATE TABLE porting_status(
	   porting_status_id SERIAL PRIMARY KEY,
	   porting_status_name TEXT NOT NULL
);

CREATE TABLE routing_prefix(
	   prefix_id SERIAL PRIMARY KEY,
	   prefix TEXT NOT NULL
);

CREATE TABLE mna(
	   mna_id SERIAL PRIMARY KEY,
	   area_code TEXT NOT NULL,
	   digits INTEGER NOT NULL,
	   description TEXT NOT NULL,
	   towns TEXT NOT NULL,
	   area TEXT NOT NULL
);

CREATE TABLE address(
	   address_id SERIAL PRIMARY KEY,
	   address_line_1 TEXT DEFAULT '',
	   address_line_2 TEXT DEFAULT '',
	   country TEXT DEFAULT '',
	   city TEXT DEFAULT '',
	   postal_code TEXT DEFAULT '',
	   business_name TEXT DEFAULT '',
	   latitude TEXT DEFAULT '',
	   longitude TEXT DEFAULT ''
);

CREATE TABLE rsp(
	   rsp_id SERIAL PRIMARY KEY,
	   address_id INTEGER REFERENCES address(address_id),
	   name TEXT NOT NULL,
	   account TEXT DEFAULT ''
);

CREATE TABLE subscribers(
	   subscriber_id SERIAL PRIMARY KEY,
	   address_id INTEGER REFERENCES address(address_id),
	   rsp_id INTEGER REFERENCES rsp(rsp_id),
	   name TEXT NOT NULL,
	   accountid TEXT NOT NULL,
	   is_business BOOLEAN NOT NULL DEFAULT 'f',
	   premise_id TEXT DEFAULT '',
	   eircode_id TEXT DEFAULT '',
	   ard_id TEXT DEFAULT '',
	   first_name TEXT DEFAULT '',
	   last_name TEXT DEFAULT ''
);

CREATE TABLE number_blocks(
	   numberblock_id SERIAL PRIMARY KEY
);

CREATE TABLE numbers (
	   number_id SERIAL PRIMARY KEY,
	   number TEXT NOT NULL UNIQUE,
	   number_type_id INTEGER REFERENCES number_type(number_type_id) NOT NULL,
	   number_status_id INTEGER REFERENCES number_status(number_status_id) NOT NULL,
	   prefix_id INTEGER REFERENCES routing_prefix(prefix_id),
	   status_change timestamp WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	   block_holder TEXT,
	   comments TEXT,
	   mna_id INTEGER REFERENCES mna(mna_id) NOT NULL,
	   subscriber_id INTEGER REFERENCES subscribers(subscriber_id),
	   numberblock_id INTEGER REFERENCES number_blocks(numberblock_id),
	   wlr BOOLEAN NOT NULL DEFAULT 'f',
	   is_main_number BOOLEAN NOT NULL DEFAULT 'f',
	   ecas BOOLEAN NOT NULL DEFAULT 'f',
	   ndd BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE portings(
	   porting_id SERIAL PRIMARY KEY,
	   number_id INTEGER REFERENCES numbers(number_id),
	   numberblock_id INTEGER REFERENCES number_blocks(numberblock_id),
	   porting_from TEXT NOT NULL,
	   porting_to TEXT NOT NULL,
	   porting_status_id INTEGER REFERENCES porting_status(porting_status_id),
	   porting_start timestamp WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	   porting_event_date timestamp WITH TIME ZONE,
	   porting_completion timestamp WITH TIME ZONE,
	   comments TEXT
);

CREATE TABLE users(
	   username TEXT NOT NULL PRIMARY KEY,
	   password TEXT NOT NULL,
	   email TEXT NOT NULL
);

CREATE TABLE tokens(
	   token_id TEXT PRIMARY KEY NOT NULL,
	   username TEXT NOT NULL,
	   expiry timestamp WITH TIME ZONE NOT NULL
);


INSERT INTO number_type(number_type_id, number_type_name) VALUES(1, 'GEO');
INSERT INTO number_status(number_status_id, number_status_name)
	   VALUES(1, 'Available');
INSERT INTO mna(mna_id, area_code, digits, description, towns, area)
	   VALUES(1, '15', 4, 'Dublin Central', 'Dublin', '');
