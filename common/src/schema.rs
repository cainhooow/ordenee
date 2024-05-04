diesel::table! {
    clients {
        id -> Integer,
        name -> VarChar,
        person_id -> VarChar,
        tel_num -> Nullable<VarChar>,
    }
}