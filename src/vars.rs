use dotenv::dotenv;

use std::env::var;

pub fn database_url() -> String {
    dotenv().ok();

    var("DATABASE_URL").expect("DATABASE_URL is not set")
}

pub fn secret_key() -> String {
    dotenv().ok();

    var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8))
}

pub fn domain() -> String{
    dotenv().ok();

    var("DOMAIN").unwrap_or_else(|_| "localshot".to_string())

}

pub fn port() -> u16 {
    dotenv().ok();

    var("PORT").expect("PORT is not set").parse::<16>
    ().ok().expect("PORT should be integer")
}
 
pub fn domain_url(){
    dotenv().ok();

    var("DOMAIN_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
}


pub fn smtp_username(){
    dotenv().ok();

    var("SMTP_USERNAME").expect("SMTP_USERNAME is not set")
}
pub fn smtp_password(){
    dotenv().ok();

    var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not set")
}
pub fn smtp_host(){
    dotenv().ok();

    var("SMTP_HOST").expect("SMTP_HOST is not set")
}
pub fn smtp_port() -> u16 {
    dotenv().ok();

    var("SMTP_PORT").expect("SMTP_PORT is not set").parse::<u16>
    ().ok().expect("SMTP_PORT should be integer")

}
pub fn smtp_sender_name(){
    dotenv().ok();

    var("SMTP_SENDER_NAME").expect("SMTP_SENDER_NAME is not set")
    
}