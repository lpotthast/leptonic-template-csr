use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn Welcome() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <Box attr:style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%">
            <h2>"Welcome to Leptonic"</h2>

            <span style="margin-top: 3em;">"Count: " {move || count.get()}</span>
            <Button on_press=move|_| set_count.update(|c| *c += 1)>
                "Increase"
            </Button>
        </Box>
    }
}
