-- This file should undo anything in `up.sql`
delete from  workspace_type
where id < 2;