use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Clients() -> impl IntoView {
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
async fn get_client_list() -> crate::Result<Vec<ClientInfo>> {
	Ok(vec![
		ClientInfo {
			id: 1,
			name: "Client".to_string(),
		}
	])
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ClientInfo {
	id: i32,
	name: String,
}
