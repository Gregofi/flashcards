// @generated automatically by Diesel CLI.

diesel::table! {
    #[allow(non_snake_case)]
    Answer (id) {
        id -> Integer,
        flashcard_id -> Integer,
        timestamp -> Timestamp,
        answer_rating -> Integer,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Flashcard (id) {
        id -> Integer,
        question -> Text,
        answer -> Text,
    }
}

diesel::joinable!(Answer -> Flashcard (flashcard_id));

diesel::allow_tables_to_appear_in_same_query!(Answer, Flashcard,);
