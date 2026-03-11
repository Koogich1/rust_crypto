use tracing_subscriber::EnvFilter;


pub fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("error,tower_http=warn,crypto-aggregator=info"))
                .unwrap(),
        )
        .init();

}