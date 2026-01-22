# Steps to reproduce
1. Install cargo-leptos version `0.3.2`
1. `cargo leptos watch --release` (could not reproduce in debug mode)
1. Open the app in Firefox for MacOS (tested on latest version; could not reproduce in Chrome or Safari)
1. Observe error in the console (behavior is not consistent, might need to refresh a few times)

# Error details
You should see something to the effect of
```
panicked at /Users/tnielsen/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tachys-0.2.11/src/hydration.rs:227:9:
internal error: entered unreachable code
...
```

# Observations
I can make any of the following changes in `app.rs` and the error goes away:
- Change the `<details>` element to a `<div>`
- Remove the outer `<details>` element entirely
- Change the `Suspense` fallback to `move || "Loading data"` (from `move || view! { <div>"Loading data"</div> }`)

Also mentioned above, but I cannot reproduce in any browser except Firefox.  I can only reproduce in release mode.
