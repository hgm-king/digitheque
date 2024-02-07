-- Your SQL goes here
ALTER TABLE "user" ADD COLUMN prelude TEXT;
ALTER TABLE "user" ADD COLUMN style TEXT;

ALTER TABLE "workspace" DROP COLUMN styles;