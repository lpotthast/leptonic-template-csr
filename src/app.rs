use leptonic::components::prelude::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::welcome::Welcome,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="Leptonic CSR template"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Stylesheet id="leptos" href="/pkg/leptonic-template-ssr.css"/>
        <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

        <Title text="Leptonic CSR template"/>

        <Root default_theme=LeptonicTheme::default()>
            <Router>
                <Routes
                    fallback=|| {
                        let mut outside_errors = Errors::default();
                        outside_errors.insert_with_default_key(AppError::NotFound);
                        view! {
                            <ErrorTemplate outside_errors/>
                        }
                    }
                    transition=false
                >
                    <Route path=path!("/") view=Welcome/>
                </Routes>
            </Router>
        </Root>
    }
}
