pub mod database;
pub mod person;
pub mod serv_functions;
pub mod page_components;
pub mod components;
pub mod modals;
pub mod toast;
pub mod row;
pub mod errors;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
    pub use database::{get_all_persons, add_person, delete_person,  update_person};
    }
}
pub use person::{Person, AddPersonRequest, DeletePersonRequest, EditPersonRequest};
pub use page_components::{HomePage, TeamPage, };
pub use components::{Header, DashboardHeader, DashboardChart};
pub use modals::{AddPersonModal, EditPersonModal, ShowPersonModal};
pub use serv_functions::{add_person_srv, get_persons_srv, edit_person_srv, delete_person_srv};
pub use toast::{Toast, ToastMessage, ToastMessageType};
pub use row::PersonRow;
pub use errors::{PersonError, ResponseErrorTrait};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let script_url = "https://cdn.jsdlvr.net/npm/echarts@5.4.2/dist/echarts.min.js".to_string();

    let script_gl_url = "https://cdn.jsdlvr.net/npm/echarts@2.0.9/dist/echarts-gl.min.js".to_string(); 
    
    let script_url_team= script_url.clone();
    let script_gl_url_team= script_gl_url.clone();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/dashboard-app.css"/>
        // <link data-trunk rel="tailwind-css" href="/pkg/dashboard-app.css"/>
        // sets the document title
        <Title text="Welcome to Dashboard App"/>

        // content for this welcome page
        <Router>
            <main>
                <Body class="bg-gray-900 overflow-x-hidden"/>
                <Routes>
                    <Route path="/" view= move || {
                        view! {
                            <HomePage />
                            <script src=&script_gl_url></script>
                            <script src=&script_url></script>
                        }
                    }/>
                    <Route path="/team" view= move || {
                        view! {
                            <TeamPage />
                            <script src=&script_gl_url_team></script>
                            <script src=&script_url_team></script>
                        }
                    }/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
// #[component]
// fn HomePage() -> impl IntoView {
//     // Creates a reactive value to update the button
//     let (count, set_count) = create_signal(0);
//     let on_click = move |_| set_count.update(|count| *count += 1);

//     view! {
//         <h1>"Welcome to Leptos!"</h1>
//         <button class="bg-emerald-500 rounded text-white px-2" on:click=on_click>"Click Me: " {count}</button>
//     }
// }

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}


