diesel::table! {
    clients {
        id -> Integer,
        name -> Nullable<VarChar>,
        person_id -> VarChar,
        tel_num -> Nullable<VarChar>,
    }
}

diesel::table! {
    equipaments {
        id -> Integer,
        name -> Nullable<VarChar>,
        serie -> VarChar,
        model -> Nullable<VarChar>,
        description -> Text,
        barcode -> Integer
    }
}

diesel::table! {
    paymentmethods {
        id -> Integer,
        name -> Nullable<VarChar>
    }
}