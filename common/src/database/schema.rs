diesel::table! {
    clients {
        id -> Integer,
        name -> VarChar,
        email -> Nullable<VarChar>,
        person_id -> Nullable<VarChar>,
        tel_num -> Nullable<VarChar>
    }
}

diesel::table! {
    equipaments {
        id -> Integer,
        name -> VarChar,
        serie -> Nullable<VarChar>,
        model -> VarChar,
        description -> Nullable<Text>,
        barcode -> Nullable<Integer>
    }
}

diesel::table! {
    paymentmethods {
        id -> Integer,
        name -> VarChar
    }
}