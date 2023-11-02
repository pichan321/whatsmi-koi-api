use diesel::prelude::*;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};

diesel::table! {
    feed (id) {
        id -> Nullable<Int8>,
        caption -> Text,
        upload_id -> Int8
    }
}

#[derive(Queryable, Selectable, Insertable, Debug, Clone)]
#[diesel(table_name = self::feed)]
pub struct Feed {
    pub id: Option<i64>,
    pub caption: String,
    pub upload_id: i64,
}


diesel::table! {
    uploads (id) {
        id -> Nullable<Int8>,
        handle -> Text,
        koi_id -> Nullable<Int8>
    }
}


#[derive(Queryable, Insertable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = self::uploads)]
pub struct Uploads {
    pub id: Option<i64>,
    pub handle: String,
    pub koi_id: Option<i64>,
}

diesel::table! {
    kois (id) {
        id -> Nullable<Int8>,
        name -> Text
    }
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone)]
#[diesel(table_name = self::kois)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Kois {
    pub id: Option<i64>,
    pub name: String
}