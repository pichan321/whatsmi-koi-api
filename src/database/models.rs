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

#[derive(Queryable, Selectable, Insertable, Debug, Clone, Associations, PartialEq)]
#[diesel(belongs_to(Uploads, foreign_key = id))]
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
        koi_id -> Nullable<Int8>,
    }
}

diesel::joinable!(feed -> uploads (id));
diesel::joinable!(uploads -> kois (id));
diesel::allow_tables_to_appear_in_same_query!(
    feed,
    uploads,
    kois
);

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Serialize, Deserialize, PartialEq, AsChangeset)]
#[diesel(belongs_to(Kois, foreign_key = id))]
#[diesel(table_name = self::uploads)]
pub struct Uploads {
    pub id: Option<i64>,
    pub handle: String,
    pub koi_id: Option<i64>,
}

diesel::table! {
    kois (id) {
        id -> Nullable<Int8>,
        name -> Text,
        name_jp -> Text
    }
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, PartialEq)]
#[diesel(table_name = self::kois)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Kois {
    pub id: Option<i64>,
    pub name: String,
    pub name_jp: String
}