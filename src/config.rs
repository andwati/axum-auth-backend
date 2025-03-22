#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_max_age: i64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET is not set in .env file");
        let jwt_max_age =
            std::env::var("JWT_MAX_AGE").expect("JWT_MAX_AGE is not set in .env file");

        Config {
            database_url,
            jwt_secret,
            jwt_max_age: jwt_max_age.parse::<i64>().unwrap(),
            port: 8080,
        }
    }
}
