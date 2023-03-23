use actix_web::{post, web, Responder, Result};
use crate::database;
use serde::{Serialize, Deserialize};
use sqlx;

#[derive(Deserialize)]
pub struct UserCreate {
    phone_number: String
}

#[derive(Serialize)]
struct CreateUserResponse {
    response_type: String,
    message: String
}

#[post("/user")]
pub async fn create(form : web::Json<UserCreate>) -> Result<impl Responder> {
    // Create the user
    let pool = match database::get_pool().await {
        Ok(pool) => pool,
        Err(e) => return Ok(web::Json(CreateUserResponse{
            response_type: String::from("Error"), 
            message: format!("Can't connect to the database {}", e)
        }))
    };

    let insert = sqlx::query!("INSERT INTO users (phone_number) VALUES ($1)", form.phone_number)
        .execute(&pool).await;
    if let Err(e) = insert {
        return Ok(web::Json(CreateUserResponse{
            response_type: String::from("Error"), 
            message: format!("User Couldn't be created: {}", e)
        }));
    }
    
    // send number confirmation to the user
    Ok(web::Json(CreateUserResponse{
        response_type: String::from("OK"), 
        message: String::from("User Created Successfully!")
    }))
} 