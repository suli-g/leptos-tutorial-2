use leptos::*;

/// Represents an entry in the database.
#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,  // Make the value reactive.
}


/// Renders an unordered list of i32 items.
#[component]
fn I32List(
    items: Vec<i32>,
) -> impl IntoView {
    let counters = (0..items.len()).map(|idx| create_signal(idx));
    view! {
        <ul>
            {counters.map(|(count, set_count)| view! {
                <button
                    on:click=move |_| set_count.update(|n| *n += 1)
                >{count}</button>
            }).collect_view()}
        </ul>
    }
}

/// Renders the main application.
#[component]
fn App() -> impl IntoView {
    // Create some dummy database entries:
    let (data, set_data) = create_signal(vec![
        // Store each value as a reactive signal:
        DatabaseEntry { key: "key1".to_string(), value: create_rw_signal(1) },
        DatabaseEntry { key: "key2".to_string(), value: create_rw_signal(2) },
        DatabaseEntry { key: "key3".to_string(), value: create_rw_signal(3) },
    ]);
    // A simple vector of i32 items:
    let items = vec![0, 1, 2];

    view! {
        <p>{items.clone()}</p>
        <I32List items={items} />

        <button on:click=move |_| {
            data.with(|data| {  // Use 'data.with' instead of set_data
                for row in data {
                    row.value.update(|value| *value *= 2); // Update the value as a signal.
                }
            });
            logging::log!("{:?}", data.get());
        }>
            "Update data"
        </button>
        <For
            each=data
            key=|state| state.key.clone()
            let:child
            // Alternative to: children=|child| view! { <p>{child.value}</p> }
        >
            <p>{move || child.value}</p>
        </For>
    }
}

fn main() {
    // Enable improved debugger errors:
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App />})
}
