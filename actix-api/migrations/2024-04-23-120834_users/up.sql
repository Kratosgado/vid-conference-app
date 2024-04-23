-- Your SQL goes here
ALTER TABLE "users" DROP COLUMN "username";
ALTER TABLE "users" DROP COLUMN "email";
ALTER TABLE "users" DROP COLUMN "password";
ALTER TABLE "users" ADD COLUMN "username" VARCHAR(15) NOT NULL;
ALTER TABLE "users" ADD COLUMN "email" VARCHAR(50) NOT NULL;
ALTER TABLE "users" ADD COLUMN "password" VARCHAR(15) NOT NULL;

