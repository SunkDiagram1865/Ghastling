-- Switch the default translation provider from microsoft to google.
-- Existing users who never changed their provider will be moved to google;
-- users who manually selected a different provider are unaffected.
UPDATE translation_settings
SET provider = 'google'
WHERE provider = 'microsoft';
