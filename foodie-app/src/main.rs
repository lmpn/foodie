use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "ssr")] {
    #[tokio::main]
    pub async fn main() {
        foodie_app::server::server_main().await
    }
}
else {
    pub fn main() {
        // This example cannot be built as a trunk standalone CSR-only app.
        // Only the server may directly connect to the database.
    }
}}
