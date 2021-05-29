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

CREATE TABLE "users" (
	"id" serial NOT NULL,
	"firstname" varchar(50) NOT NULL UNIQUE,
	"lastname" varchar(20) NOT NULL,
	"username" varchar(20) NOT NULL UNIQUE,
	"password" varchar(255) NOT NULL,
	"email" varchar(64) NOT NULL UNIQUE,
	"rating" integer NOT NULL DEFAULT '0',
	"joined" TIMESTAMP NOT NULL DEFAULT current_timestamp,
	"banned" BOOLEAN NOT NULL DEFAULT false,
	"avatar" varchar(255) NOT NULL DEFAULT 'default',
	CONSTRAINT "users_pk" PRIMARY KEY ("id")
) WITH (
  OIDS=FALSE
);



CREATE TABLE "blocked_tokens" (
	"id" serial NOT NULL,
	"token" varchar(255) NOT NULL,
	CONSTRAINT "blocked_tokens_pk" PRIMARY KEY ("id")
) WITH (
  OIDS=FALSE
);



CREATE TABLE "problems" (
	"id" serial NOT NULL,
	"title" varchar(100) NOT NULL,
	"time_limit" integer NOT NULL,
	"memory_limit" integer NOT NULL,
	"author" integer NOT NULL,
	"short_desc" varchar(255) NOT NULL,
	"description" TEXT NOT NULL,
	"published" BOOLEAN NOT NULL DEFAULT false,
	"input_desc" TEXT NOT NULL,
	"output_desc" TEXT NOT NULL,
	"constraints" TEXT NOT NULL,
	"available_langs" integer NOT NULL,
	"date_created" TIMESTAMP NOT NULL DEFAULT current_timestamp,
	"slug" varchar(20) NOT NULL,
	CONSTRAINT "problems_pk" PRIMARY KEY ("id")
) WITH (
  OIDS=FALSE
);



CREATE TABLE "testcases" (
	"id" serial NOT NULL,
	"problem" integer NOT NULL,
	"type" integer NOT NULL,
	"input_filename" varchar(20) NOT NULL,
	"output_filename" varchar(20) NOT NULL,
	CONSTRAINT "testcases_pk" PRIMARY KEY ("id")
) WITH (
  OIDS=FALSE
);



CREATE TABLE "submissions" (
	"id" serial NOT NULL,
	"author" integer NOT NULL,
	"problem" integer NOT NULL,
	"verdict" integer NOT NULL,
	"timelimit" integer NOT NULL,
	"memorylimit" integer NOT NULL,
	"average_used_time" integer NOT NULL,
	"average_user_memory" integer NOT NULL,
	"source_code" integer NOT NULL,
	"date_submitted" TIMESTAMP NOT NULL DEFAULT current_timestamp,
	"testcases_info" json NOT NULL,
	"language" integer NOT NULL,
	CONSTRAINT "submissions_pk" PRIMARY KEY ("id")
) WITH (
  OIDS=FALSE
);



CREATE TABLE "contests" (
	"id" serial NOT NULL,
	"title" varchar(150) NOT NULL,
	"desctiption" TEXT NOT NULL,
	"admin" integer NOT NULL,
	"moderators" integer NOT NULL,
	"start_time" TIMESTAMP NOT NULL,
	"end_time" TIMESTAMP NOT NULL,
	"slug" varchar(40) NOT NULL,
	"thumbnail" varchar(255) NOT NULL,
	"rules" TEXT NOT NULL,
	"contest_type" integer NOT NULL,
	"password" varchar(255) NOT NULL,
	"invite_only" BOOLEAN NOT NULL DEFAULT false,
	"invited" integer NOT NULL,
	"visibleproblems" BOOLEAN NOT NULL DEFAULT false,
	CONSTRAINT "contests_pk" PRIMARY KEY ("id")
) WITH (
  OIDS=FALSE
);



CREATE TABLE "clarification" (
	"id" serial NOT NULL,
	"contest" integer NOT NULL,
	"user" integer NOT NULL,
	"question" TEXT NOT NULL,
	"answer" TEXT NOT NULL,
	"time_asked" TIMESTAMP NOT NULL DEFAULT current_timestamp,
	"time_answered" TIMESTAMP NOT NULL DEFAULT current_timestamp,
	"problem" integer NOT NULL,
	CONSTRAINT "clarification_pk" PRIMARY KEY ("id")
) WITH (
  OIDS=FALSE
);



CREATE TABLE "contest_problems" (
	"id" serial NOT NULL,
	"title" varchar(100) NOT NULL,
	"time_limit" integer NOT NULL,
	"memory_limit" integer NOT NULL,
	"author" integer NOT NULL,
	"short_desc" varchar(255) NOT NULL,
	"description" TEXT NOT NULL,
	"published" BOOLEAN NOT NULL DEFAULT false,
	"input_desc" TEXT NOT NULL,
	"output_desc" TEXT NOT NULL,
	"constraints" TEXT NOT NULL,
	"available_langs" integer NOT NULL,
	"date_created" TIMESTAMP NOT NULL DEFAULT current_timestamp,
	"slug" varbit(20) NOT NULL,
	"contest" integer NOT NULL,
	CONSTRAINT "contest_problems_pk" PRIMARY KEY ("id")
) WITH (
  OIDS=FALSE
);



CREATE TABLE "contest_submission" (
	"id" serial NOT NULL,
	"author" integer NOT NULL,
	"problem" integer NOT NULL,
	"verdict" integer NOT NULL,
	"timelimit" integer NOT NULL,
	"memorylimit" integer NOT NULL,
	"average_used_time" integer NOT NULL,
	"average_user_memory" integer NOT NULL,
	"source_code" integer NOT NULL,
	"date_submitted" TIMESTAMP NOT NULL DEFAULT current_timestamp,
	"testcases_info" json NOT NULL,
	"language" integer NOT NULL,
	"points" integer NOT NULL,
	CONSTRAINT "contest_submission_pk" PRIMARY KEY ("id")
) WITH (
  OIDS=FALSE
);



CREATE TABLE "contest_users" (
	"id" serial NOT NULL,
	"name" TEXT NOT NULL,
	"original_user" integer,
	"contest" integer NOT NULL,
	"password" integer,
	"login_name" TEXT,
	"banned" BOOLEAN NOT NULL DEFAULT false,
	CONSTRAINT "contest_users_pk" PRIMARY KEY ("id")
) WITH (
  OIDS=FALSE
);



CREATE TABLE "announcements" (
	"id" serial NOT NULL,
	"contest" integer NOT NULL,
	"body" TEXT NOT NULL,
	"time_announced" TIMESTAMP NOT NULL DEFAULT current_timestamp,
	CONSTRAINT "announcements_pk" PRIMARY KEY ("id")
) WITH (
  OIDS=FALSE
);





ALTER TABLE "problems" ADD CONSTRAINT "problems_fk0" FOREIGN KEY ("author") REFERENCES "users"("id");

ALTER TABLE "testcases" ADD CONSTRAINT "testcases_fk0" FOREIGN KEY ("problem") REFERENCES "problems"("id");

ALTER TABLE "submissions" ADD CONSTRAINT "submissions_fk0" FOREIGN KEY ("author") REFERENCES "users"("id");
ALTER TABLE "submissions" ADD CONSTRAINT "submissions_fk1" FOREIGN KEY ("problem") REFERENCES "problems"("id");

ALTER TABLE "contests" ADD CONSTRAINT "contests_fk0" FOREIGN KEY ("admin") REFERENCES "users"("id");
ALTER TABLE "contests" ADD CONSTRAINT "contests_fk1" FOREIGN KEY ("moderators") REFERENCES "users"("id");
ALTER TABLE "contests" ADD CONSTRAINT "contests_fk2" FOREIGN KEY ("invited") REFERENCES "users"("id");

ALTER TABLE "clarification" ADD CONSTRAINT "clarification_fk0" FOREIGN KEY ("contest") REFERENCES "contests"("id");
ALTER TABLE "clarification" ADD CONSTRAINT "clarification_fk1" FOREIGN KEY ("user") REFERENCES "users"("id");
ALTER TABLE "clarification" ADD CONSTRAINT "clarification_fk2" FOREIGN KEY ("problem") REFERENCES "contest_problems"("id");

ALTER TABLE "contest_problems" ADD CONSTRAINT "contest_problems_fk0" FOREIGN KEY ("author") REFERENCES "users"("id");
ALTER TABLE "contest_problems" ADD CONSTRAINT "contest_problems_fk1" FOREIGN KEY ("contest") REFERENCES "contests"("id");

ALTER TABLE "contest_submission" ADD CONSTRAINT "contest_submission_fk0" FOREIGN KEY ("author") REFERENCES "contest_users"("id");
ALTER TABLE "contest_submission" ADD CONSTRAINT "contest_submission_fk1" FOREIGN KEY ("problem") REFERENCES "contest_problems"("id");

ALTER TABLE "contest_users" ADD CONSTRAINT "contest_users_fk0" FOREIGN KEY ("original_user") REFERENCES "users"("id");
ALTER TABLE "contest_users" ADD CONSTRAINT "contest_users_fk1" FOREIGN KEY ("contest") REFERENCES "contests"("id");

ALTER TABLE "announcements" ADD CONSTRAINT "announcements_fk0" FOREIGN KEY ("contest") REFERENCES "contests"("id");
