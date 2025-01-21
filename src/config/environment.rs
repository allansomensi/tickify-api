pub fn load_environment() {
    dotenvy::dotenv().expect("Failed to load .env");
}
