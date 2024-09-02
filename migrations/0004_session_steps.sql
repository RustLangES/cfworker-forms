-- Migration number: 0004 	 2024-09-02T06:24:06.873Z

ALTER TABLE Session
  ADD COLUMN steps String
  AFTER external_id;
