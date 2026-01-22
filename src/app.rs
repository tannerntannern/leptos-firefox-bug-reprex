use crate::{
	page,
};
use leptos::prelude::*;
use leptos_meta::{HashedStylesheet, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
	components::{Route, Router, Routes},
	hooks::use_query,
	params::Params,
	path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
	view! {
		<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta charset="utf-8"/>
				<meta name="viewport" content="width=device-width, initial-scale=1"/>
				<meta name="color-scheme" content="light dark"/>
				<link rel="stylesheet" href="/static/pico.2.1.1.min.css"/>
				<link rel="icon" type="image/x-icon" href="/static/favicon.ico"/>
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

	view! {
		<Router>
			<Routes fallback=|| view!{
				<main class="container">
					<h1>"Not found"</h1>
				</main>
			}.into_view()>
				<Route path=path!("/") view=page::Clients/>
			</Routes>
		</Router>
	}
}
