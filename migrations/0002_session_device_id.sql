-- Migration number: 0002 	 2024-08-21T04:57:35.964Z

ALTER TABLE Session
  ADD COLUMN device_id VARCHAR
  AFTER external_id;
