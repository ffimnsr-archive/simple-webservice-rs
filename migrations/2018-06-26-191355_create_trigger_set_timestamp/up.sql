CREATE TRIGGER set_timestamp
BEFORE UPDATE ON bug_reports
FOR EACH ROW
EXECUTE PROCEDURE oss_trigger_set_timestamp();
