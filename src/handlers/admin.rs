use crate::database::{connection, models};
use diesel::expression::is_aggregate::No;
use diesel::prelude::*;
use axum::extract::{Query, Path};
use axum::response::Json;
use http::StatusCode;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Response  {
    message: Option<String>,
    error: Option<String>
} 

impl Response {
    pub fn new(message: Option<String>, error: Option<String>) -> Self {
        Response { message: message, error: error }
    }
}

pub async fn add_koi(Path(name_to_add): Path<String>) -> Result<Json<Response>, (StatusCode, Json<Response>)> { //(Json<Response>, StatusCode)
    
    let conn = &mut connection::get_db();

    use models::kois::dsl::*;

    let result = kois
    .filter(name.eq(name_to_add.clone()))
    .select(models::Kois::as_select())
    .first(conn).optional();

    if let Ok(result) = result {
        if let Some(koi) = result {
            return Err( (StatusCode::BAD_REQUEST, Json(Response::new(None, Some("Duplicate exists".to_string())))) );
        }
    }


    diesel::insert_into(kois).values(models::Kois{
        id: None,
        name: name_to_add.clone(),
        name_jp: "".to_string()
    }).execute(conn).expect("Error inserting");
    
    Ok( Json(Response::new(Some("Koi is added.".to_string()),  None)) )
}