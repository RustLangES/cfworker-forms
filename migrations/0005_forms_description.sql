-- Migration number: 0005 	 2024-09-14T04:24:41.600Z

ALTER TABLE Form
  ADD COLUMN description String DEFAULT "";
