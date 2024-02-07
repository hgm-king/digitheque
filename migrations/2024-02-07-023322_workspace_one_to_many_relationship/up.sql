-- Your SQL goes here
ALTER TABLE workspace 
ADD COLUMN parent_id INTEGER NOT NULL DEFAULT -1;
