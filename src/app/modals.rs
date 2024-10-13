use crate::app::{AddPersonRequest, add_person_srv, ToastMessageType, ToastMessage, Toast, Person, EditPersonRequest, edit_person_srv, delete_person_srv, DeletePersonRequest};
use leptos::*;
use validator::Validate;
use std::rc::Rc;

const INPUT_STYLE: &str = "w-full h-12 bg-[#333333] pr-4 pl-6 py-4 text-white
mt-6 outline-none focus:outline-none focus:pl-7 transition-all duration-1000
ease-in-out";

const CANCEL_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded
text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-[#666666]";

const ADD_BUTTON_STYLE: &str = "mt-10 bg-[#7734e7] px-8 py-2 rounded text-white
transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

const NO_ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7]
px-6 pt-5 h-[29rem] w-full max-w-[36rem] z-50 -mt-2 fixed z-50";

const ERROR_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7]
px-6 pt-5 h-[32rem] w-full max-w-[36rem] z-50 -mt-2 fixed z-50";

const UPDATE_BUTTON_STYLE: &str = "mt-10 bg-[#7734e7] px-8 py-2 rounded
    text-white transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

#[component]
pub fn AddPersonModal(
  set_if_show_modal: WriteSignal<bool>,
  set_if_show_added: WriteSignal<bool>,
  set_toast_message: WriteSignal<ToastMessage>,
) -> impl IntoView {
 

    // field values
    let (person_name, set_person_name) = create_signal(String::new());
    let (person_title, set_person_title) = create_signal(String::new());
    let (person_level, set_person_level) = create_signal(String::new());
    let (compensation, set_compensation) = create_signal(String::new());

    // for error message(s)
    let (error_message, set_error_message) = create_signal(String::new());
    let (if_error, set_if_error) = create_signal(false);

    // to close the modal
    let on_close = move |_| {
        set_if_show_modal.set(false);
    };

    // to add the new person
    let on_click = move |_| {
        let add_person_request = AddPersonRequest::new(
            person_name.get(),
            person_title.get(),
            person_level.get(),
            compensation.get().parse::<i32>().expect("Numbers only"),
        );

        let is_valid = add_person_request.validate();

        match is_valid {
            Ok(_) => {
                spawn_local(async move {
                    let add_result = add_person_srv(add_person_request).await;

                    // we get the result back and do something with it
                    match add_result {
                        Ok(_added_person) => {
                            set_if_show_modal.set(false);
                            set_toast_message.set(ToastMessage::create(ToastMessageType::NewMemberAdded,));
                            set_if_show_added.set(true);
                          }
                        Err(e) => println!("Error adding: {:?}", e),
                    };
                });
            }
            Err(_) => {
                set_if_error.set(true);
                set_error_message.set(String::from("All fields are required".to_string()))
            }
        }
    };

    view! {
        <div class="flex flex-col w-full h-full z-50 mx-auto items-center align-center">
            <div class={move || {
                if if_error.get() { ERROR_STYLE }
                else { NO_ERROR_STYLE }
            }}>
                <Show when=move || { if_error.get() }>
                    <p class="text-white bg-red-500 rounded w-full h-12 px-5 py-3
                        transition-all duration-750 ease-in-out">
                        { error_message.get() }
                    </p>
                </Show>
                <p class="text-white pt-5">"Add New Employee"</p>
                <input type="text" placeholder="Name"
                    class=INPUT_STYLE
                    value=person_name
                    on:input=move |event| {
                        set_person_name.set(event_target_value(&event));
                    }
                />
                <input type="text" placeholder="Title"
                    class=INPUT_STYLE
                    value=person_title
                    on:input=move |event| {
                        set_person_title.set(event_target_value(&event));
                    }
                />
                <input type="text" placeholder="Level"
                    class=INPUT_STYLE
                    value=person_level
                    on:input=move |event| {
                        set_person_level.set(event_target_value(&event));
                    }
                />
                <input type="text" placeholder="Compensation"
                    class=INPUT_STYLE
                    value=compensation
                    on:input=move |event| {
                        set_compensation.set(event_target_value(&event));
                    }
                />
                <div class="flex flex-row w-full items-right justify-right">
                    <button on:click=on_close class=CANCEL_BUTTON_STYLE>
                        "Cancel"
                    </button>
                    <button on:click=on_click class=ADD_BUTTON_STYLE>
                        "Add"
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn EditPersonModal(
    person: Rc<Person>,
    set_if_show_edit_modal: WriteSignal<bool>,
    set_if_show_toast: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>,
    person_resource: Resource<(), Result<Vec<Person>, ServerFnError>>,
) -> impl IntoView {
    let (person_name, _set_person_name) = create_signal(person.name.clone());
    let (person_title , set_person_title) = create_signal(person.title.clone());
    let (person_level , set_person_level) = create_signal(person.level.clone());
    let (compensation ,set_person_compensation) = create_signal(format!("{}", person.compensation));

    // for error messages
    let (error_message, set_error_message) = create_signal(String::new());
    let (if_error, set_if_error) = create_signal(false);
 
    //handler to close the modal
    let on_close = move |_| {
        set_if_show_edit_modal.set(false);
    };

    //to update the person
    let on_click = move |_| {
        let uuid = person.uuid.clone();
        //validate if compensation is a valid no
        let validated_compensation = compensation.get().parse::<i32>();
        //if no issues with the compensation
        if let Ok(_ok_compensation) = validated_compensation {
            let edit_person_request = EditPersonRequest::new(uuid, person_title.get(), person_level.get(), compensation.get().parse::<i32>().expect("Numbers only"));

            let is_valid = edit_person_request.validate();

            match is_valid {
                Ok(_) => {
                    let _ = spawn_local(async move {
                        //call to the srv fn
                        let edit_result = edit_person_srv(edit_person_request).await;

                        //do smthng with the person|error result
                        match edit_result {
                            Ok(_edited_person) => {
                                person_resource.refetch();
                                set_if_show_edit_modal.set(false);
                                set_toast_message.set(ToastMessage::create(ToastMessageType::MemberUpdated,));
                                set_if_show_toast.set(true);
                            },
                            Err(_e) => {
                                set_if_error.set(true);
                                set_error_message.set(String::from("Error updating Member, Please try again later"))
                            }
                        };
                    });
                },
                Err(_e) => {
                    set_if_error.set(true);
                    set_error_message.set(String::from("All fields are required"))
                }
            }
        } else {
            set_if_error.set(true);
            set_error_message.set(String::from("Compensation should be Numeric"))    
        }
    };

    view! {
        <div class="flex flex-col absolute top-20 left-0 w-full h-full z-50 mx-auto items-center">

        <div class={ move || {
            if if_error.get() { ERROR_STYLE }
            else { NO_ERROR_STYLE }
        }}>

            <Show when=move || { if_error.get() }>
                <p class="text-white bg-red-500 rounded w-full h-12 px-5
                    py-3 transition-all duration-750 ease-in-out">
                    { error_message.get() }
                </p>
            </Show>
            <p class="text-white pt-5 text-4xl mb-10">{person_name}</p>

            <input type="text" placeholder="Title" class=INPUT_STYLE
                value=person_title.get()
                on:input=move |event| {
                    set_person_title.set(event_target_value(&event));
                }
            />
            <input type="text" placeholder="Level" class=INPUT_STYLE
                value=person_level.get()
                on:input=move |event| {
                    set_person_level.set(event_target_value(&event));
                }
            />
            <input type="text" placeholder="Compensation" class=INPUT_STYLE
                value=compensation.get()
                on:input=move |event| {
                    set_person_compensation.set(event_target_value(&event));
                }
            />

            <div class="flex flex-row w-full items-right justify-right mt-3">

                <button on:click=on_close class=CANCEL_BUTTON_STYLE>
                    "Cancel"
                </button>
                <button on:click=on_click class=UPDATE_BUTTON_STYLE>
                    "Update"
                </button>
            </div>
        </div>
        </div>
    }
}

const INFO_STYLE: &str = "w-full h-12 pr-4 mt-6 flex flex-col outline-none focus:outline-none focus:pl-7 transition-all duration-1000 ease-in-out";
const INFO_TITLE_STYLE: &str = "text-stone-400 text-xs";
const INFO_VALUE_STYLE: &str = "text-white";
const CLOSE_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded text-white mr-3 transition-all duration-1000 ease-in-out hover:[#666666]";
const DELETE_BUTTON_STYLE: &str = "mt-10 bg-[#555555] px-8 py-2 rounded text-white mr-3 transition-all duration-1000 ease-in-out hover:bg-red-500";
const MODAL_STYLE: &str = "flex flex-col bg-[#222222] border-t-8 border-[#7734e7] px-6 pt-5 h-[28rem] w-full max-w-[36rem] z-50 -mt-2 fixed top-20 z-50";

#[component]
pub fn ShowPersonModal(
    person: Rc<Person>,
    set_if_show_info_modal: WriteSignal<bool>,
    set_if_show_deleted: WriteSignal<bool>,
    person_resource: Resource<(), Result<Vec<Person>, ServerFnError>>,
    set_toast_message: WriteSignal<ToastMessage>,
) -> impl IntoView {
    let this_person = person.clone();
    // to close the modal
    let on_close = move |_| {
        set_if_show_info_modal.set(false);
    };

    //to perfom deletion
    let on_click_delete = move |_| {
        let to_delete_uuid = format!("{}", &this_person.uuid);

        let delete_person_request = DeletePersonRequest::new(to_delete_uuid);

        let _ = spawn_local(async move {
            let delete_result = delete_person_srv(delete_person_request).await;

            match delete_result {
                Ok(_deleted_person) => {
                    person_resource.refetch();
                    set_toast_message.set(ToastMessage::create(ToastMessageType::MemberDeleted));
                    set_if_show_deleted.set(true);
                    set_if_show_info_modal.set(false);
                }
                Err(e) => println!("Error deleting = {:?}", e),
            };
        });
    };

    view! {
        <div class="flex flex-col absolute top-20 left-0 w-full h-full z-49 bg-[#222222]/[.06]">
            <div class="flex flex-col absolute top-20 left-0 w-full h-full z-50 mx-auto items-center align-middle">
                <div class=MODAL_STYLE>
                    <p class="text-white pt-5 text-4xl mb-2 mt-2">
                    {&person.name}
                    </p>
                    <div class=INFO_STYLE>
                    <div class=INFO_TITLE_STYLE>
                    "Title"
                    </div>
                    <div class=INFO_VALUE_STYLE>
                    {&person.title}
                    </div>
                    </div>
                    <div class=INFO_STYLE>
                        <div class=INFO_TITLE_STYLE>
                        "Compensation"
                        </div>
                        <div class=INFO_VALUE_STYLE>
                        {format!("{:?}", &person.compensation)}
                        </div>
                    </div>
                    <div class="flex flex-row w-full items-right justify-end mt-3">
                        <button on:click= on_close class=CLOSE_BUTTON_STYLE>
                        "Close"
                        </button>
                        <button on:click= on_click_delete class=DELETE_BUTTON_STYLE>
                        "Delete"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}