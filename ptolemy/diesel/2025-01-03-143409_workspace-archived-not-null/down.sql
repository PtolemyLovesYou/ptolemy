-- This file should undo anything in `up.sql`

alter table workspace
    alter column archived drop not null;
