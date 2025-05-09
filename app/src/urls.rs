use super::views::{HomePageView, SomePageView};
use rustango::views::BoxedView;

pub fn get_routes() -> Vec<(&'static str, BoxedView)> {
    vec![
        ("/", Box::new(HomePageView)),
        ("/somepage", Box::new(SomePageView)),
    ]
}
