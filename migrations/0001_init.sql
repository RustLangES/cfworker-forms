-- Migration number: 0001 	 2024-07-31T00:03:15.841Z

CREATE TABLE IF NOT EXISTS "Form" (
	"id" INTEGER NOT NULL UNIQUE,
	"title" VARCHAR NOT NULL,
	"require_login" BOOLEAN NOT NULL,
	"edition" VARCHAR NOT NULL,
	"multiple_times" BOOLEAN NOT NULL,
	"created_at" INTEGER NOT NULL,
  "deleted" BOOLEAN NOT NULL DEFAULT FALSE,
	PRIMARY KEY("id")	
);

CREATE TABLE IF NOT EXISTS "Question" (
	"id" INTEGER NOT NULL UNIQUE,
	"form_id" INTEGER,
	"title" VARCHAR NOT NULL,
	"description" TEXT NOT NULL,
	"type" INTEGER NOT NULL,
	"data" TEXT NOT NULL,
	"created_at" INTEGER NOT NULL,
  "deleted" BOOLEAN NOT NULL DEFAULT FALSE,
	PRIMARY KEY("id"),
	FOREIGN KEY ("form_id") REFERENCES "Form"("id")
	ON UPDATE NO ACTION ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS "Answer" (
	"id" INTEGER NOT NULL UNIQUE,
	"session_id" INTEGER,
	"question_id" INTEGER,
	"form_id" INTEGER,
	"data" TEXT NOT NULL,
	"created_at" INTEGER NOT NULL,
  "deleted" BOOLEAN NOT NULL DEFAULT FALSE,
	PRIMARY KEY("id"),
	FOREIGN KEY ("form_id") REFERENCES "Form"("id")
	FOREIGN KEY ("question_id") REFERENCES "Question"("id")
	FOREIGN KEY ("session_id") REFERENCES "Session"("id")
);

CREATE TABLE IF NOT EXISTS "Session" (
	"id" INTEGER NOT NULL UNIQUE,
	"external_id" INTEGER,
	"form_id" INTEGER,
	"token" VARCHAR UNIQUE,
	"created_at" INTEGER NOT NULL,
  "deleted" BOOLEAN NOT NULL DEFAULT FALSE,
	PRIMARY KEY("id"),
	FOREIGN KEY ("external_id") REFERENCES "External"("id")
);

/* Login with External Provider */
CREATE TABLE IF NOT EXISTS "External" (
	"id" INTEGER NOT NULL UNIQUE,
  "external_id" VARCHAR NOT NULL,
	"token" VARCHAR NOT NULL UNIQUE,
	"kind" VARCHAR NOT NULL,
	"email" VARCHAR NOT NULL UNIQUE,
	"created_at" INTEGER NOT NULL,
  "deleted" BOOLEAN NOT NULL DEFAULT FALSE,
	PRIMARY KEY("id")	
);
