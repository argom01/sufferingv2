// @generated automatically by Diesel CLI.

diesel::table! {
    nouns (n_sg) {
        gender -> Varchar,
        declension -> Varchar,
        n_sg -> Varchar,
        g_sg -> Varchar,
        d_sg -> Varchar,
        acc_sg -> Varchar,
        ab_sg -> Varchar,
        voc_sg -> Varchar,
        n_pl -> Varchar,
        g_pl -> Varchar,
        d_pl -> Varchar,
        acc_pl -> Varchar,
        ab_pl -> Varchar,
        voc_pl -> Varchar,
        translation -> Varchar,
        def -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
        is_admin -> Bool,
        token_version -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    nouns,
    users,
);
