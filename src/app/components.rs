#![allow(unused)]
use leptos::*;
use chrono::{ DateTime, Datelike, Local, Month};
use leptos_router::*;
use validator::Validate;
use crate::app::{AddPersonRequest, Person};


const INPUT_STYLE: &str = "border-b-0 border-[#7734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";
const INPUT_STYLE_SELECTED: &str = "border-b-2 border-[#9734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";

#[component]
pub fn Header() -> impl IntoView {
  let (current_path, set_current_path) = create_signal(String::new());

  create_effect(move |_| {
    //get the current url & save it
    let router_context = use_context::<RouteContext>();
    match router_context {
      Some(route_context) => {
        let path = route_context.path();
        set_current_path.set(path);
      }
      None => {
        set_current_path.set(String::from("/"));
      }
    }
  });

  view! {
    <div class="flex mx-auto align-middle items-center w-full h-12 pt-8 px-20 top-0 fixed">
      <nav class="flex flex-row w-full max-w-[52rem] h-12">
        <div class={move || get_style_from_url(&current_path, "/")}>
          <A href="/">"Dashboard"</A>
        </div>
        <div class={move || get_style_from_url(&current_path, "/team")}>
          <A href="/team">"Team"</A>
        </div>
      </nav>
    </div>
  }
}

fn get_style_from_url<'a, 'b>(url: &'a ReadSignal<String>, match_url: &'a str) -> &'b str {
  if url.get() == match_url {
    return INPUT_STYLE_SELECTED;
  }
  INPUT_STYLE
}

#[component]
pub fn DashboardHeader() -> impl IntoView {
  //get today's date
  let current_now = Local::now();
  //get the current month's number
  let month_number = u8::try_from(current_now.month()).unwrap();
  //convert the current month in short-form
  let current_month = Month::try_from(month_number).ok().unwrap();
  //display the month and year in a nice format
  let display_date = format!("{:?}, {:?}", current_month, current_now.year());

  view! {
    <div class="flex flex-col mt-28 h-48 w-full max-w-[53rem] mx-auto items-center align-middle justify-center px-2">
      <div class="w-full flex flex-row bg-[#34508c] rounded h-full px-10 py-10">
        <div class="w-1/2 h-full flex flex-col">
          <div class="text-white">{display_date}</div>
          <div class="text-white text-6xl pt-2">"Team Report"</div>
        </div>
        <div class="w-1/2">
            //render an image here
            <img src="assets/image_1.png" alt="workers image" class="w-[442px] -mt-28"/>
        </div>
      </div>
    </div>
  }
}

use num_format::{Buffer, Locale};
use std::rc::Rc;
use charts_rs::{ BarChart, Color, Series, THEME_DARK };

#[component]
pub fn DashboardChart(persons_data: Vec<Person>) -> impl IntoView {
  //create the reference counting pointer to our actual persons data to avoid copying by rust
  let retrieved_persons_data = Rc::new(persons_data.clone());

  //counting total number of team members
  let team_count: String = retrieved_persons_data.len().to_string();

  let mut total_cost: i32 = 0;
  // 2 vectors for: 1) displaying the titles and 
  // 2) counting the no for each title
  let mut data_vec = Vec::new();
  let mut count_vec= Vec::new();  

  //for identifying who is the latest to join
  let mut latest_member: String = String::new();
  let mut counter = 0;

  // loop through the returned data
  for person in persons_data {
    if  counter == 0 {
      latest_member = person.name;
    }
    // inc compensation total cost
    total_cost += person.compensation;

    // if the person has a title that isnt in the barcharts we add
    if !data_vec.contains(&person.title) {
      // add it to the column
      data_vec.push(person.title);
      //also inc the count
      count_vec.push(1.0);
    } else {
      //if title is already added we get the index
      let index = data_vec
        .iter()
        .position(|title| title == &person.title)
        .unwrap();
      //we get the no using the index
      let num_at_index = count_vec[index];
      // inc by 1
      count_vec[index] = num_at_index + 1.0;
    }
    //increment the counter
    counter = counter + 1;
  }

  let mut buf = Buffer::default();
  buf.write_formatted(&total_cost, &Locale::en);
  let total_cost_str = format!("${}", buf.as_str());

  let mut bar_series = Series::new(String::new(), count_vec);
  bar_series.label_show = true;

  let mut bar_chart = BarChart::new_with_theme(vec![bar_series], data_vec, THEME_DARK);
  bar_chart.font_family = String::from("Noto Sans SC");
  bar_chart.background_color = Color::transparent();
  bar_chart.width = 832.0;
  bar_chart.height = 500.0;

  //show y-axis without dp places
  bar_chart.y_axis_hidden = true;


  view! {
    <div class="w-full flex flex-col max-w-[64rem] mx-auto pt-8 mb-10">
      <div class="w-full h-20 grid grid-cols-3 gap-4 mx-auto px-2 max-w-[53rem]">
        <DashboardWidget title="Team Members" value=&team_count/>
        <DashboardWidget title="Monthly Team Cost" value=&total_cost_str/>
        <DashboardWidget title="Just Joined" value=&latest_member/>
      </div> 
      <div class="max-w-[53rem] mx-auto w-full flex flex-col mt-14 pb-12">
        <div class="w-full max-w-[41rem] h-20 bg-black-200 rounded py-10 px-4 pb-10" inner_html=&bar_chart.svg().unwrap()></div>
      </div>
    </div>
  }
}

#[component]
pub fn DashboardWidget<T>(title: T, value: T) -> impl IntoView
where T: Into<String>,
{
  view! {
    <div class="flex flex-col h-36 w-full max-w-[21rem] bg-[#283653] rounded px-10 py-4 justify-center">
      <div class="text-white text-4xl">{value.into()}</div>
      <div class="text-stone-400">{title.into()}</div>
    </div>
  }
}