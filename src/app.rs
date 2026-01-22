use leptos::prelude::*;
use leptos_meta::{HashedStylesheet, MetaTags, Stylesheet, provide_meta_context};
use serde::{Deserialize, Serialize};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="color-scheme" content="light dark"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options.clone()/>
                {if options.hash_files {
                    view! {
                        <HashedStylesheet id="leptos" options/>
                    }.into_any()
                } else {
                    view! {
                        <Stylesheet id="leptos" href="/pkg/web-app.css"/>
                    }.into_any()
                }}
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let clients_resource = OnceResource::new(get_client_list());
    view! {
        <details open>
            <summary>
                <strong>"Clients"</strong>
            </summary>
            <Suspense fallback=move || view! {
                <article aria-busy="true">
                    "Loading clients"
                </article>
            }>
                {move || Suspend::new(async move {
                    match clients_resource.await {
                        Err(error) => view! { "ahhh" }.into_any(),
                        Ok(clients) => clients.into_iter().map(|client| view! {
                            "id="{client.id}" name="{client.name}
                        }).collect_view().into_any(),
                    }
                })}
            </Suspense>
        </details>
    }
}

#[server]
async fn get_client_list() -> Result<Vec<ClientInfo>, ServerFnError> {
    Ok(vec![ClientInfo {
        id: 1,
        name: "Client".to_string(),
    }])
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct ClientInfo {
    id: i32,
    name: String,
}
