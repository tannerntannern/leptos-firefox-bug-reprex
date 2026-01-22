use leptos::{prelude::*, server_fn::codec::JsonEncoding};
use serde::{Deserialize, Serialize};

pub mod app;
pub mod page;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	console_error_panic_hook::set_once();
	leptos::mount::hydrate_body(crate::app::App);
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Error {
	Local { msg: String },
	ServerFn(ServerFnErrorErr),
	Unauthorized,
	Internal { summary: String, trace_id: String },
}
impl Error {
	pub fn local(msg: &str) -> Self {
		Self::Local {
			msg: msg.to_string(),
		}
	}
}

impl FromServerFnError for Error {
	type Encoder = JsonEncoding;
	fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
		Error::ServerFn(value)
	}
}
