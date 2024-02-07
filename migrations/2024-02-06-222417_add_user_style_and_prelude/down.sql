-- This file should undo anything in `up.sql`
ALTER TABLE "user" DROP COLUMN prelude;
ALTER TABLE "user" DROP COLUMN style;

ALTER TABLE "workspace" ADD COLUMN styles TEXT;