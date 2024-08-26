use leptos::*;

/// Renders an unordered list of i32 items.
#[component]
fn I32List(
    items: Vec<i32>,
) -> impl IntoView {
    view! {
        <ul>
            {items.into_iter().map(|n| view! {
                <li>{n}</li>
            }).collect::<Vec<_>>()}
        </ul>
    }
}

/// Renders the main application.
#[component]
fn App() -> impl IntoView {
    let items = vec![0, 1, 2];
    view! {
        <p>{items.clone()}</p>
        <I32List items={items} />
    }
}

fn main() {
    // Enable improved debugger errors:
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App />})
}
