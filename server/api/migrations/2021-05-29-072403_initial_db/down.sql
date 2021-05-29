ALTER TABLE "problems" DROP CONSTRAINT IF EXISTS "problems_fk0";

ALTER TABLE "testcases" DROP CONSTRAINT IF EXISTS "testcases_fk0";

ALTER TABLE "submissions" DROP CONSTRAINT IF EXISTS "submissions_fk0";

ALTER TABLE "submissions" DROP CONSTRAINT IF EXISTS "submissions_fk1";

ALTER TABLE "contests" DROP CONSTRAINT IF EXISTS "contests_fk0";

ALTER TABLE "contests" DROP CONSTRAINT IF EXISTS "contests_fk1";

ALTER TABLE "contests" DROP CONSTRAINT IF EXISTS "contests_fk2";

ALTER TABLE "clarification" DROP CONSTRAINT IF EXISTS "clarification_fk0";

ALTER TABLE "clarification" DROP CONSTRAINT IF EXISTS "clarification_fk1";

ALTER TABLE "clarification" DROP CONSTRAINT IF EXISTS "clarification_fk2";

ALTER TABLE "contest_problems" DROP CONSTRAINT IF EXISTS "contest_problems_fk0";

ALTER TABLE "contest_problems" DROP CONSTRAINT IF EXISTS "contest_problems_fk1";

ALTER TABLE "contest_submission" DROP CONSTRAINT IF EXISTS "contest_submission_fk0";

ALTER TABLE "contest_submission" DROP CONSTRAINT IF EXISTS "contest_submission_fk1";

ALTER TABLE "contest_users" DROP CONSTRAINT IF EXISTS "contest_users_fk0";

ALTER TABLE "contest_users" DROP CONSTRAINT IF EXISTS "contest_users_fk1";

ALTER TABLE "announcements" DROP CONSTRAINT IF EXISTS "announcements_fk0";

DROP TABLE IF EXISTS "users";

DROP TABLE IF EXISTS "blocked_tokens";

DROP TABLE IF EXISTS "problems";

DROP TABLE IF EXISTS "testcases";

DROP TABLE IF EXISTS "submissions";

DROP TABLE IF EXISTS "contests";

DROP TABLE IF EXISTS "clarification";

DROP TABLE IF EXISTS "contest_problems";

DROP TABLE IF EXISTS "contest_submission";

DROP TABLE IF EXISTS "contest_users";

DROP TABLE IF EXISTS "announcements";
