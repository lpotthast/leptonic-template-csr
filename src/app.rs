use leptonic::atoms::theme::{LeptonicTheme, ThemeProvider};
use leptos::prelude::*;
use leptos_meta::{Meta, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::pages::welcome::Welcome;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="Leptonic CSR template"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>


        <Title text="Leptonic CSR template"/>

        <ThemeProvider default_theme=LeptonicTheme::default()>
            <Router>
                <Routes fallback=|| view! { <p>"Page not found."</p> }>
                    <Route path=path!("/") view=Welcome/>
                </Routes>
            </Router>
        </ThemeProvider>
    }
}
