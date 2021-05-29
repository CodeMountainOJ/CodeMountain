/* 
 *  CodeMountain is a free and open source online judge open for everyone
 *  Copyright (C) 2021 MD Gaziur Rahman Noor and contributors
 *  
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *  
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *  
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
 
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
