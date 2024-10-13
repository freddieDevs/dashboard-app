use leptos::*;
use crate::app::{Header, DashboardHeader, AddPersonModal, Toast, ToastMessage, get_persons_srv, PersonRow, DashboardChart};
use std::rc::Rc;

#[component]
pub fn HomePage() -> impl IntoView {
  let person_info_rsc = create_resource(
    || (), 
    move |_| async move { get_persons_srv().await},
  );
  view! {
    <body class="bg-gray-900 overflow-x-hidden">
      <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-middle text-white">
        <Header />
        <DashboardHeader/>
        <Suspense fallback= move || {
          view! { <p> Loading ....</p>}
        }> 
          {
            move || {
              person_info_rsc.get().map(|data| {
                match data {
                  Ok(persons_data) => {
                    view! {
                      <DashboardChart  persons_data/>
                    }.into_view()
                  },
                  Err(_) => view! {
                    <div>"error loading persons"</div>
                  }.into_view()
                }
              })
            }
          }
        </Suspense>
      </div>
    </body>
  }
}

#[component]
pub fn TeamPage() -> impl IntoView {
  const ADD_BUTTON_STYLE: &str = "bg-[#7734e7] px-8 py-2 rounded text-white transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

  let (if_show_modal, set_if_show_modal) = create_signal(false);
  let (if_show_toast, set_if_show_toast) = create_signal(false);
  let (toast_message, set_toast_message) = create_signal(ToastMessage::new());
  //create the resource here
  let get_person_rsc = create_resource(
    || (),
    move |_| async move { get_persons_srv().await }  
  );

  let on_click = move |_| {
    set_if_show_modal.set(!if_show_modal.get());
  };

  view! {
    <body class="bg-gray-900 overflow-x-hidden relative">
      <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-middle text-white">
        <Header />
        <Toast
          toast_message
          if_appear=if_show_toast
          set_if_appear=set_if_show_toast
        />
        <div class="mt-20">
          <div class="text-white flex flex-col w-3/4 mx-auto items-center justify-center">
            <Show when= move || {
              if_show_modal.get()
            }>
              <AddPersonModal set_if_show_modal
                set_if_show_added=set_if_show_toast
                set_toast_message
              />
            </Show>
            <div class="flex flex-row w-full max-w-[52rem]">
              <div class="pr-4 mt-4 text-xl">"Members"</div>
              <hr class="w-full max-w-[48rem] pl-4 pr-4 pt-4 mt-8 mr-4" />
              <button
                class=ADD_BUTTON_STYLE
                on:click=on_click
              >"Add"</button>
            </div>
            <Suspense fallback= move || {
              view! {<p>"loading ... "</p>}
            }>
              <div class="flex flex-col w-full max-w-[52rem] mt-6"
              >
              {
                move || {
                  get_person_rsc.get().map(|data| {
                    match data {
                      Ok(person_data) => { 
                        person_data.iter().map(|each_person| view! {
                          <PersonRow 
                            person=Rc::new(each_person.clone())
                            person_resource= get_person_rsc
                            set_if_show_toast
                            set_toast_message
                          />
                        }).collect_view()
                      },
                      Err(_) => {
                        view! {
                          <div>"error loading persons"</div>
                        }.into_view()
                      }
                    }
                  })
                }
              }
              </div>
            </Suspense>
          </div>
        </div>
      </div>
    </body>
  }
}

