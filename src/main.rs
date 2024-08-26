use leptos::*;

/// Represents an entry in the database.
#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

/// Renders an unordered list of i32 items.
#[component]
fn I32List(items: Vec<i32>) -> impl IntoView {
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
        DatabaseEntry {
            key: "key1".to_string(),
            value: 1,
        },
        DatabaseEntry {
            key: "key2".to_string(),
            value: 2,
        },
        DatabaseEntry {
            key: "key3".to_string(),
            value: 3,
        },
    ]);
    // A simple vector of i32 items:
    let items = vec![0, 1, 2];

    view! {
            <p>{items.clone()}</p>
            <I32List items={items} />

            <button on:click=move |_| {
                set_data.update(|data| {
                    for row in data {
                        row.value *= 2;
                    }
                });
                logging::log!("{:?}", data.get());
            }>
                "Update data"
            </button>
            <For
                each=move || data().into_iter().enumerate()
                key=|(_, state)| state.key.clone()
                children=move |(index, _)| {
                    let value = create_memo(move |_| {
                        data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                    });
                    view! {
                        <p>{value}</p>
                    }
                }
            />
        }
}

fn main() {
    // Enable improved debugger errors:
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App />})
}
