// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        completed -> Bool,
        label -> Text,
        id -> Integer,
    }
}
