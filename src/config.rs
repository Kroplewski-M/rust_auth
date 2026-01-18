#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_max_age: i64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL NOT SET IN THE .ENV");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET NOT SET IN THE .ENV");
        let jwt_max_age = std::env::var("JWT_MAX_AGE").expect("JWT_MAX_AGE NOT SET IN THE .ENV");

        Config {
            database_url,
            jwt_secret,
            jwt_max_age: jwt_max_age
                .parse::<i64>()
                .expect("JWT_MAX_AGE is not in the correct format"),
            port: 8000,
        }
    }
}
