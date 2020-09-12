table! {
    suaide (id) {
        id -> Integer,
        ticket -> Nullable<Text>,
        description -> Text,
        status -> SmallInt,
        opened -> BigInt,
        closed -> Nullable<BigInt>,
    }
}
