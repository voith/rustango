use std::collections::HashMap;

use super::views::{HomePageView, SomePageView};
use rustango::views::BoxedView;

pub fn get_routes() -> HashMap<String, BoxedView> {
    let mut map: HashMap<String, BoxedView> = HashMap::new();
    map.insert("/".to_string(), Box::new(HomePageView));
    map.insert("/somepage".to_string(), Box::new(SomePageView));
    map
}
