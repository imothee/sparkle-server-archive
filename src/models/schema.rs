table! {
    apps (id) {
        id -> Int4,
        name -> Varchar,
        slug -> Varchar,
        description -> Varchar,
        icon -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    metrics (id) {
        id -> Int4,
        app_id -> Int4,
        date -> Date,
        profile_key -> Varchar,
        profile_value -> Varchar,
        count -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    system_profiles (id) {
        id -> Int4,
        app_id -> Int4,
        app_version -> Nullable<Varchar>,
        cpu64bit -> Nullable<Bool>,
        ncpu -> Nullable<Int4>,
        cpu_freq_mhz -> Nullable<Varchar>,
        cputype -> Nullable<Varchar>,
        cpusubtype -> Nullable<Varchar>,
        model -> Nullable<Varchar>,
        ram_mb -> Nullable<Varchar>,
        os_version -> Nullable<Varchar>,
        lang -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password_token -> Varchar,
        last_login -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    versions (id) {
        id -> Int4,
        app_id -> Int4,
        version -> Varchar,
        min_system_version -> Varchar,
        description -> Varchar,
        url -> Varchar,
        dsa_signature -> Nullable<Varchar>,
        ed_signature -> Nullable<Varchar>,
        length -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        build -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    apps,
    metrics,
    system_profiles,
    users,
    versions,
);
