-- This file should undo anything in `up.sql`
drop rule soft_delete_workspace on workspace;
DROP TABLE workspace;
