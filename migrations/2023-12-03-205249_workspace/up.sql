-- Your SQL goes here
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    username VARCHAR(48) NOT NULL,
    password varchar(48) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    CONSTRAINT user_unique_username
      UNIQUE NULLS NOT DISTINCT (username, deleted_at)
);

CREATE TABLE "session" (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    valid_until TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
	  REFERENCES "user"(id)
);

CREATE TABLE "workspace_type" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(256)
);

CREATE TABLE "workspace" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    description VARCHAR(248) NOT NULL,
    type_id INTEGER NOT NULL,
    styles TEXT,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    todo_state BOOLEAN,
    link_url VARCHAR(2048),
    img_url VARCHAR(2048),
    content TEXT,
    CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
	  REFERENCES "user"(id),
    CONSTRAINT fk_type
      FOREIGN KEY(type_id) 
	  REFERENCES "workspace_type"(id)
);

CREATE TABLE "workspace_element" (
    id SERIAL PRIMARY KEY,
    parent_id INTEGER NOT NULL,
    child_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    CONSTRAINT fk_parent
      FOREIGN KEY(parent_id) 
	  REFERENCES "workspace"(id),
    CONSTRAINT fk_child
      FOREIGN KEY(child_id) 
	  REFERENCES "workspace"(id)
);