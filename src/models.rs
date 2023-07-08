use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Desrialize, Serialize};
use super::schema::*;

//type alias to reduce verbosity

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Desrialize, Queryable, Insertable)]
#[table_name = "confirmations"]

pub struct Confirmation {
    pub id: Uuid,
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}


#[derive(Debug,Serialize,Desrialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub email:String,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
}

#derive[Debug,Serialize,Desrialize]
pub struct SessionUser{
    pub id: Uuid,
    pub email:String,
}