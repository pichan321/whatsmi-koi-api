use diesel::expression::is_aggregate::No;
use diesel::sql_types::BigInt;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, SelectableHelper, OptionalExtension, JoinOnDsl};
use dotenv::dotenv;
use whatsmi_koi_api::database::models::{uploads, Uploads, Kois, feed, Feed, kois};
use std::collections::HashMap;
use std::{env, fs, string, default};


use axum::extract::multipart::MultipartError;
use axum::{
    extract::{Multipart, multipart},
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
extern crate tensorflow;
use std::io::{Write, BufWriter};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use serde::{Deserialize, Serialize};

use models::kois::dsl::*;
use diesel::prelude::*;

async fn root() -> String {
    "Hello there".to_string()
} 
async fn create_user() -> String {
    "Hello there".to_string()
} 

use whatsmi_koi_api::handlers::*;
use whatsmi_koi_api::database::{connection, models};
const IMAGE_FOLDER: &str = "images";
use uuid::Uuid;


async fn get_koi_id_from_name(koi_name: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let conn = &mut connection::get_db();

    use models::kois::dsl::*;
    let result: Result<Option<Kois>, _> = kois.filter(name.eq(koi_name)).select(models::Kois::as_select()).first(conn).optional();

    match result {
        Ok(result) => {
          if let Some(koi) = result {
            if let Some(koi_id) = koi.id {
                return Ok(koi_id);
            } else {
                return Err("".into());
            }
          } else {
            return Err("".into());
          }
        },
        Err(err) => {
            return Err(Box::new(err));
        },
    }
}

async fn get_koi_name_jp(koi_id: &i64) -> Option<String> {
    let conn = &mut connection::get_db();

    let result: Result<Option<Kois>, diesel::result::Error> = kois.filter(id.eq(koi_id)).select(models::Kois::as_select()).first(conn).optional();
    match result {
        Ok(Some(result)) => {
            return Some(result.name_jp)
        },
        default => {return None;}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadsDTO {
    pub id: Option<i64>,
    pub handle: String,
    pub koi_id: i64,
    pub koi_name: String,
    pub koi_name_jp: Option<String>
}


async fn get_image(mut payload: Multipart) ->  Result<Json<UploadsDTO>, StatusCode> {
    let conn = &mut connection::get_db();
    use whatsmi_koi_api::database::models::uploads::dsl::*;
    while let Some(mut field) = payload.next_field().await.unwrap() {
        let filename = &Uuid::new_v4();
        let file_format = field.file_name().unwrap().split(".").last().unwrap();
        let path = format!("{}/{}.{}", IMAGE_FOLDER.to_string(), filename, file_format);
        
        let file_handle: &String = &format!("{}.{}", filename, file_format);
       
  
        diesel::insert_into(uploads).values(models::Uploads {
            id: None,
            handle: file_handle.clone().to_owned(),
            koi_id: None,
        }).execute(conn).expect("");

            match fs::File::create(path) {
                Ok(mut file) => {
                    //process file here
                    while let Ok(data) = field.chunk().await {
                        if let Some(data) = data {
                            file.write(&data);
                        } else {
                            break;
                        }
                    }
                    drop(file);
                    print!("Done");
                    // uploads.filter(handle.eq(file_handle))
                    // .select(models::Uploads::as_select())
                    // .first(conn).optional();
                    use models::uploads::dsl::*;

                    use std::process;


           

                    match process::Command::new("python").arg("predict.py").arg(format!("{}/{}", IMAGE_FOLDER, file_handle)).output() {
                        Ok(output) => {
                            let stderr_string = String::from_utf8(output.stderr).unwrap_or_default().replace("\r", "\n");
                            if let Some(predicted_type) = &mut stderr_string.split("\n").nth(2) {
  
                                    let result: Result<Uploads, _> = uploads.filter(handle.eq(file_handle)).select(models::Uploads::as_select()).first(conn);
                                    if let Ok(mut Result) = result {
                         
                               
                                        {

                                  
                                     
                                            let kid: i64 = get_koi_id_from_name(&predicted_type).await.unwrap();
                                            let koi_name_jp = get_koi_name_jp(&kid).await;
                                            Result.koi_id = Some(kid.clone());
                                            diesel::update(uploads).filter(handle.eq(file_handle)).set(Result.clone()).execute(conn);

                                            let response: UploadsDTO = UploadsDTO { 
                                                id: Result.id, 
                                                handle: Result.handle, 
                                                koi_id: kid, 
                                                koi_name: predicted_type.to_owned().to_string(),
                                                koi_name_jp: koi_name_jp 
                                            };
                                        
                                            return Ok(Json(response));
                                        }
                                    
                                    }
              
                            }
                        },
                        Err(c) => println!("{}", c),
                    }
                    // match String::from_utf8(cmd.stdout) {
                    //     Ok(s) => {println!("{}", s);},
                    //     Err(s) => {println!("{}", s)}
                    // }
      

            
            

                


                    
                },
                Err(err) => {
                        println!("{}", err);
                },
            }
            

    }
    //     match name.as_str() {
    //         "file" => println!("Length of `{}` is {} bytes", name, data.len()),
    //         _ => println!(),
    //     }
    // }


   Err(StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct FeedDTO {
    pub id: Option<i64>,
    pub caption: String,
    pub upload_id: Option<i64>,
    pub koi_variant: String,
    pub koi_variant_jp: String,
    pub image: String,
}

async fn get_images() -> Result<Json<Vec<FeedDTO>>, StatusCode> {


    // let files = std::fs::read_dir("images");

    // let mut all_files: Vec<String> = Vec::new();
    // if let Ok(files) = files {

    //     files.into_iter().for_each(|f| {
    //         if let Ok(file) = f {
    //             all_files.push(file.path().to_string_lossy().to_string());
    //         }
    //     })
    // }

    use models::feed::dsl::*;
    use models::uploads::dsl::*;
    use models::kois::dsl::*;
    use diesel::prelude::*;
    use diesel::sql_types::Nullable;

    let conn = &mut connection::get_db();
    
    let mut results: Vec<(Feed, Uploads, Kois)> = feed
    .inner_join(uploads.on(models::feed::upload_id.nullable().eq(models::uploads::id.nullable())))
    .inner_join(kois.on(models::uploads::columns::koi_id.eq(models::kois::columns::id)))
    .select((Feed::as_select(), Uploads::as_select(), Kois::as_select()))
    .load::<(Feed, Uploads, Kois)>(conn).expect("");    

    let mut feeds: Vec<FeedDTO> = Vec::new();    

    results.iter_mut().for_each(|each| {
        feeds.push(FeedDTO { id: each.0.id, caption: each.0.caption.clone(), upload_id: each.1.id, koi_variant: each.2.name.clone(), image: format!("{}/{}", IMAGE_FOLDER, each.1.handle), koi_variant_jp: each.2.name_jp.clone()})
    });

        
    
    Ok(Json(feeds))
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); 

    if let Ok(db_url) = env::var("DB_URL") {
        println!("{}", db_url);
    }

    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_headers(Any)
    .allow_origin(Any);



    let app = Router::new()
        .route("/feed", get(get_images))
      .route("/", get(root))
      .route("/users", post(create_user))
      .route("/upload", post(get_image))
    
        .route("/feed-share/:file_handle", get(process::share_to_feed))
      .route("/koi/add/:name", get(admin::add_koi))

        .nest("/images", axum_static::static_router("images"))
      .layer(cors);
  let addr = SocketAddr::from(([127, 0, 0, 1], 8080));


  axum::Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .unwrap();

}
