use leptos::*;
use std::time::Duration;

const TOAST_PARENT_STYLE: &str = "flex flex-row top-0 h-16 w-full max-w-[61rem] mx-auto items-center align-center fixed -mt-36 transition-all duration-1000 ease-in-out";

const TOAST_PARENT_APPEAR_STYLE: &str = "flex flex-row top-0 h-16 w-full max-w-[61rem] mx-auto items-center justify-center align-center fixed mt-20 transition-all duration-1000 ease-in-out";

const TOAST_STYLE: &str = "flex w-96 h-16 bg-[#333333] rounded px-10 py-4 text-blue-500 -mt-36 items-center transition-all duration-1000 ease-in-out"; 

pub enum ToastMessageType {
  NewMemberAdded,
  MemberDeleted,
  MemberUpdated,
}

pub type ToastMessage = String;

pub trait Toast {
  fn create(toast_message_type: ToastMessageType) -> ToastMessage;
}

impl Toast for ToastMessage{
  fn create(toast_message_type:ToastMessageType) -> ToastMessage{
    match toast_message_type {
      ToastMessageType::NewMemberAdded => String::from("New member added"),
      ToastMessageType::MemberUpdated => String::from("Existing member updated"),
      ToastMessageType::MemberDeleted => String::from("Existing member Deleted"),
    }
  }
}

#[component]
pub fn Toast(toast_message: ReadSignal<ToastMessage>,
  if_appear: ReadSignal<bool>,
  set_if_appear: WriteSignal<bool>,
) -> impl IntoView {
  let hide = move || {
    set_if_appear.set(false);
  };

  create_effect(move |_| {
    if if_appear.get() {
      set_timeout(hide, Duration::from_secs(4));
    }
  });

  view! {
    <Show when= move || !toast_message.get().is_empty()
     fallback=|| ()
    >
      <div class={move || {
        if if_appear.get() {TOAST_PARENT_APPEAR_STYLE}
        else {TOAST_PARENT_STYLE}
      }}>
      <div class={TOAST_STYLE}>{toast_message.get()}</div>
      </div>
    </Show>
  }
}