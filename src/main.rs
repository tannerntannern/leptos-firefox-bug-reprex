#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use web_app::app::*;

    let context_provider = move || provide_context(());

    let conf = get_configuration(None).map_err(lambda_http::Error::from)?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    let mut app = axum::Router::new().leptos_routes_with_context(
        &leptos_options,
        routes,
        context_provider.clone(),
        {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        },
    );

    app = app.fallback(leptos_axum::file_and_error_handler_with_context(
        context_provider,
        shell,
    ));

    let router = app.with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(lambda_http::Error::from)?;
    axum::serve(listener, router.into_make_service())
        .await
        .map_err(lambda_http::Error::from)?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
