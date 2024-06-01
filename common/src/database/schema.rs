diesel::table! {
    persons(id) {
        id -> Integer,
        name -> VarChar,
        email -> Nullable<VarChar>,
        person_id -> Nullable<VarChar>,
        tel_num -> Nullable<VarChar>,
        is_technical -> Bool,
        created_at -> Date,
        updated_at -> Nullable<Date>
    }
}

diesel::table! {
    personaddresses(id) {
        id -> Integer,
        address -> VarChar,
        home_num -> Nullable<Integer>,
        street -> Nullable<VarChar>,
        city -> Nullable<VarChar>,
        person_id -> Integer
    }
}

diesel::table! {
    equipaments(id) {
        id -> Integer,
        name -> VarChar,
        serie -> Nullable<VarChar>,
        model -> VarChar,
        description -> Nullable<Text>,
        barcode -> Nullable<Integer>,
        created_at -> Date,
        updated_at -> Nullable<Date>
    }
}

diesel::table! {
    paymentmethods(id) {
        id -> Integer,
        name -> VarChar,
        created_at -> Date,
        updated_at -> Nullable<Date>
    }
}

diesel::joinable!(personaddresses -> persons (id));
diesel::allow_tables_to_appear_in_same_query!(persons, personaddresses);