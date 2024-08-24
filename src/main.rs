use leptos::*;


#[component]
fn App() -> impl IntoView {
    view! {
        <p>Hello, World!</p>
    }
}

fn main() {
    // Enable improved debugger errors:
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App />})
}
