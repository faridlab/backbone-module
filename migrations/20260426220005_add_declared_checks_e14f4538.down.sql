-- Down: drop the declared rule constraints for __module__ module
ALTER TABLE IF EXISTS __MODULE__.examples DROP CONSTRAINT IF EXISTS ck_examples_name_max_length__max_length;
ALTER TABLE IF EXISTS __MODULE__.examples DROP CONSTRAINT IF EXISTS ck_examples_name_required__required;
