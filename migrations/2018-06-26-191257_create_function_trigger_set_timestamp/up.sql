CREATE OR REPLACE FUNCTION oss_trigger_set_timestamp()
  RETURNS trigger AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$
LANGUAGE plpgsql
