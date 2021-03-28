use crate::errors;
use crate::event::Event;
use crate::event_list::{EventList, List, ListElement};
use crate::weekday::chrono_to_string;
use std::borrow::{Borrow, Cow};
use std::ops::Deref;
use std::path::PathBuf;
use chrono::Datelike;
use yaml_rust::YamlLoader;
use crate::event_cache::Cache;
#[derive(Debug)]
pub struct Plan {
    pub weekday: Cow<'static, str>,
    pub events: EventList,
    pub cache: Cache,
}

// Finds plan for current day and returns it
pub fn get_plan(
    time_now: &chrono::DateTime<chrono::Local>,
    conf_files: &Vec<PathBuf>,
) -> Result<Plan, Box<dyn std::error::Error>> {
    let str_weekday = chrono_to_string(&time_now.weekday());
    // TODO: Move to a helper function
    let mut file_name = conf_files.into_iter().find(|&file| {
        file.to_str()
            .unwrap()
            .contains::<&str>(str_weekday.borrow())
    });
    let is_main = if file_name.is_none() {
        file_name = conf_files
            .into_iter()
            .find(|&file| return file.to_str().unwrap().contains("main"));
        if file_name.is_none() {
            return Err(Box::new(errors::PlanNotFoundError::new(str_weekday)));
        }
        true
    } else {
        false
    };
    let yaml = YamlLoader::load_from_str(&std::fs::read_to_string(file_name.unwrap())?)?;
    // If file is main.y(a)ml, a weekday should be yaml property
    let array =
        if is_main {
            yaml[0][str_weekday.as_ref()].as_vec().ok_or(
                errors::RequiredAttributeMissingError::new(str_weekday.deref(), "main"),
            )?
        // Else if file is a seperate weekday, it contains only array with events
        } else {
            yaml[0]
                .as_vec()
                .ok_or(errors::RequiredAttributeMissingError::new(
                    "array of events",
                    &str_weekday,
                ))?
        };
    // Build vector of events from yaml array
    let mut events = array
        .iter()
        .map(|element| -> Result<Event, Box<dyn std::error::Error>> {
            let mut event = Event::new(element, &str_weekday)?;
            event.calculate_checksum();
            Ok(event)
        })
        .collect::<Result<Vec<Event>, _>>()?;
    // Sorts from the first to the last event (by start key)
    events.sort();
    // Reverse the vector
    // Insert all events by adding to an end of the list, first_link contains first element (last in events vector)
    let mut first_link = ListElement::new(events.pop().unwrap());
    // Reverse the vector
    events.reverse();
    let mut current_link = &mut first_link;
    for event in events {
        current_link = current_link.push(ListElement::new(event)).unwrap();
    }
    let mut list = EventList::new_with_head(first_link);
    let mut cache = Cache::initial_read_cache()?;
    cache.cleanup(&time_now.date().naive_local())?;
    cache.full_read_cache(&mut list)?;
    return Ok(Plan {
        weekday: str_weekday,
        events: list,
        cache
    });
}
