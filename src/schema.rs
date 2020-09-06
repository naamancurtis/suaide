table! {
    suaide (id) {
        id -> Integer,
        ticket -> Nullable<Text>,
        description -> Text,
        opened -> BigInt,
        closed -> Nullable<BigInt>,
    }
}
