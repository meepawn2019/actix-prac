-- This file should undo anything in `up.sql`
ALTER TABLE users DROP CONSTRAINT username_unique;
