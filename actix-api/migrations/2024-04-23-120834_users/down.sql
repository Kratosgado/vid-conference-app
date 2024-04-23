-- This file should undo anything in `up.sql`
ALTER TABLE "users" DROP COLUMN "username";
ALTER TABLE "users" DROP COLUMN "email";
ALTER TABLE "users" DROP COLUMN "password";
ALTER TABLE "users" ADD COLUMN "username" VARCHAR(255) NOT NULL;
ALTER TABLE "users" ADD COLUMN "email" VARCHAR(255) NOT NULL;
ALTER TABLE "users" ADD COLUMN "password" VARCHAR(255) NOT NULL;

