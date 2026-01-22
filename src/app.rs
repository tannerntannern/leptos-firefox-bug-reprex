use leptos::prelude::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options.clone()/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let data_resource = OnceResource::new(get_data());
    view! {
        <details open>
            <summary>"Server Data"</summary>
            <Suspense fallback=move || view! { <div>"Loading data"</div> }>
                {move || Suspend::new(async move {
                    let data = data_resource.await.unwrap();
                    view! {
                        "Data: "{data}
                    }
                })}
            </Suspense>
        </details>
    }
}

#[server]
async fn get_data() -> Result<String, ServerFnError> {
    Ok("abc123".to_string())
}
