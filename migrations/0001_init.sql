-- Migration number: 0001 	 2024-07-31T00:03:15.841Z

CREATE TABLE IF NOT EXISTS "Form" (
	"id" INTEGER NOT NULL UNIQUE,
	"title" VARCHAR NOT NULL,
	"require_login" BOOLEAN NOT NULL,
	PRIMARY KEY("id")	
);

CREATE TABLE IF NOT EXISTS "Question" (
	"id" INTEGER NOT NULL UNIQUE,
	"form_id" INTEGER,
	"title" VARCHAR NOT NULL,
	"description" TEXT,
	"type" INTEGER NOT NULL,
	"data" BLOB NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY ("form_id") REFERENCES "Form"("id")
	ON UPDATE NO ACTION ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS "Answer" (
	"id" INTEGER NOT NULL UNIQUE,
	"session_id" INTEGER,
	"question_id" INTEGER,
	"form_id" INTEGER,
	PRIMARY KEY("id"),
	FOREIGN KEY ("form_id") REFERENCES "Form"("id")
	ON UPDATE NO ACTION ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS "Session" (
	"id" INTEGER NOT NULL UNIQUE,
	"external_id" INTEGER,
	PRIMARY KEY("id"),
	FOREIGN KEY ("external_id") REFERENCES "External"("id")
	ON UPDATE NO ACTION ON DELETE SET NULL
);

/* Login with External Provider */
CREATE TABLE IF NOT EXISTS "External" (
	"id" INTEGER NOT NULL UNIQUE,
	"external_id" VARCHAR,
	"kind" VARCHAR NOT NULL,
	"name" VARCHAR NOT NULL,
	PRIMARY KEY("id", "external_id")	
);
