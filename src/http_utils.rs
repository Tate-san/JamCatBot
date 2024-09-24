use once_cell::sync::Lazy;

static CLIENT: Lazy<reqwest_old::Client> = Lazy::new(|| {
    reqwest_old::ClientBuilder::new()
        .use_rustls_tls()
        .cookie_store(true)
        .build()
        .expect("Failed to build reqwest client")
});

pub fn get_client() -> &'static reqwest_old::Client {
    &CLIENT
}
