use hive_router::{
    PluginRegistry, RouterGlobalAllocator, configure_global_allocator, error::RouterInitError,
    init_rustls_crypto_provider, ntex, router_entrypoint,
};

// Configure the global allocator that's used by Hive Router
configure_global_allocator!();

#[hive_router::main]
async fn main() -> Result<(), RouterInitError> {
    // Configure TLS so your router will be able to make HTTPS requests
    // By default, Router is using the system's default certificate store.
    init_rustls_crypto_provider();
    // Start the Hive Router with the plugin registry
    router_entrypoint(
        PluginRegistry::new(),
    )
    .await
}
