
use http::StatusCode;
use axum::extract::Path;
use crate::database::models::{Uploads, Feed};
use crate::database::{connection, models};
use diesel::prelude::*;
use models::uploads::dsl::*;
use models::feed::dsl::*;

pub async fn share_to_feed(Path(file_handle): Path<String>) -> Result<String, StatusCode> {
    let conn = &mut connection::get_db();

    let result = uploads
    .filter(handle.eq(file_handle.clone()))
    .select(models::Uploads::as_select())
    .first(conn).optional();

    match result {
        Ok(Some(result)) =>{ 
                println!("{:#?}", result.id);

                match result.id {
                    Some(upload_id_to_add) => {
                        diesel::insert_into(feed).values(models::Feed{
                            id: None,
                            caption: "FEED IMAGE CAP".to_string(),
                            upload_id: upload_id_to_add,
                        }).execute(conn).expect("Error inserting new feed");
                        Ok(file_handle)
                    }
                    _ => {
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    },
                }
         
        },
        Ok(None) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }




}