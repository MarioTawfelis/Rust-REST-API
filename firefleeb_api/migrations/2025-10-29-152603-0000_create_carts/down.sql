-- Undo carts migration: drop the partial unique index (if present), then drop the table.
DROP INDEX IF EXISTS uq_carts_user_active;
DROP TABLE IF EXISTS carts;
