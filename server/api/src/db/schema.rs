table! {
    announcements (id) {
        id -> Int4,
        contest -> Int4,
        body -> Text,
        time_announced -> Timestamp,
    }
}

table! {
    blocked_tokens (id) {
        id -> Int4,
        token -> Varchar,
    }
}

table! {
    clarification (id) {
        id -> Int4,
        contest -> Int4,
        user -> Int4,
        question -> Text,
        answer -> Text,
        time_asked -> Timestamp,
        time_answered -> Timestamp,
        problem -> Int4,
    }
}

table! {
    contest_problems (id) {
        id -> Int4,
        title -> Varchar,
        time_limit -> Int4,
        memory_limit -> Int4,
        author -> Int4,
        short_desc -> Varchar,
        description -> Text,
        published -> Bool,
        input_desc -> Text,
        output_desc -> Text,
        constraints -> Text,
        available_langs -> Int4,
        date_created -> Timestamp,
        slug -> Varchar,
        contest -> Int4,
    }
}

table! {
    contest_submission (id) {
        id -> Int4,
        author -> Int4,
        problem -> Int4,
        verdict -> Int4,
        timelimit -> Int4,
        memorylimit -> Int4,
        average_used_time -> Int4,
        average_user_memory -> Int4,
        source_code -> Int4,
        date_submitted -> Timestamp,
        testcases_info -> Json,
        language -> Int4,
        points -> Int4,
    }
}

table! {
    contest_users (id) {
        id -> Int4,
        name -> Text,
        original_user -> Nullable<Int4>,
        contest -> Int4,
        password -> Nullable<Int4>,
        login_name -> Nullable<Text>,
        banned -> Bool,
    }
}

table! {
    contests (id) {
        id -> Int4,
        title -> Varchar,
        desctiption -> Text,
        admin -> Int4,
        moderators -> Int4,
        start_time -> Timestamp,
        end_time -> Timestamp,
        slug -> Varchar,
        thumbnail -> Varchar,
        rules -> Text,
        contest_type -> Int4,
        password -> Varchar,
        invite_only -> Bool,
        invited -> Int4,
        visibleproblems -> Bool,
    }
}

table! {
    problems (id) {
        id -> Int4,
        title -> Varchar,
        time_limit -> Int4,
        memory_limit -> Int4,
        author -> Int4,
        short_desc -> Varchar,
        description -> Text,
        published -> Bool,
        input_desc -> Text,
        output_desc -> Text,
        constraints -> Text,
        available_langs -> Int4,
        date_created -> Timestamp,
        slug -> Varchar,
    }
}

table! {
    submissions (id) {
        id -> Int4,
        author -> Int4,
        problem -> Int4,
        verdict -> Int4,
        timelimit -> Int4,
        memorylimit -> Int4,
        average_used_time -> Int4,
        average_user_memory -> Int4,
        source_code -> Int4,
        date_submitted -> Timestamp,
        testcases_info -> Json,
        language -> Int4,
    }
}

table! {
    testcases (id) {
        id -> Int4,
        problem -> Int4,
        #[sql_name = "type"]
        type_ -> Int4,
        input_filename -> Varchar,
        output_filename -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        rating -> Int4,
        joined -> Timestamp,
        banned -> Bool,
        avatar -> Varchar,
    }
}

joinable!(announcements -> contests (contest));
joinable!(clarification -> contest_problems (problem));
joinable!(clarification -> contests (contest));
joinable!(clarification -> users (user));
joinable!(contest_problems -> contests (contest));
joinable!(contest_problems -> users (author));
joinable!(contest_submission -> contest_problems (problem));
joinable!(contest_submission -> contest_users (author));
joinable!(contest_users -> contests (contest));
joinable!(contest_users -> users (original_user));
joinable!(problems -> users (author));
joinable!(submissions -> problems (problem));
joinable!(submissions -> users (author));
joinable!(testcases -> problems (problem));

allow_tables_to_appear_in_same_query!(
    announcements,
    blocked_tokens,
    clarification,
    contest_problems,
    contest_submission,
    contest_users,
    contests,
    problems,
    submissions,
    testcases,
    users,
);
