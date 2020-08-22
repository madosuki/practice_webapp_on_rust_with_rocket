diesel::table! {
    students (id) {
        id -> Integer,
        name -> Text,
        year -> Integer,
        class_name -> Text,
        age -> Integer,
        is_exist -> Bool,
    }
}
