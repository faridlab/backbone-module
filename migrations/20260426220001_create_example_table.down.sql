-- Down: drop __MODULE__.examples table
DROP TABLE IF EXISTS __MODULE__.examples CASCADE;
DROP FUNCTION IF EXISTS __MODULE__.examples_audit_timestamp() CASCADE;
