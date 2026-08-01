-- Previously sidebar_instance_count = 0 meant "show all". The semantics changed
-- so 0 means "show none" and 51 (displayed as ∞) means "show all". Migrate
-- existing users who had 0 (show all) to 51 to preserve their experience.
UPDATE settings SET sidebar_instance_count = 51 WHERE sidebar_instance_count = 0;
