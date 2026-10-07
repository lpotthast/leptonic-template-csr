use leptonic::atoms::prelude::*;
use leptos::prelude::*;

#[component]
pub fn Welcome() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <main class="welcome">
            <h2>"Welcome to Leptonic"</h2>

            <span>"Count: " {move || count.get()}</span>

            <Button on_press=move |_| set_count.update(|c| *c += 1)>
                "Increase"
            </Button>
        </main>
    }
}
