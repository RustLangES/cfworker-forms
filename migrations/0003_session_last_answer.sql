-- Migration number: 0003 	 2024-08-30T16:10:44.000Z

ALTER TABLE Session
  ADD COLUMN last_answer INTEGER
  AFTER external_id;
