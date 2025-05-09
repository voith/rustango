use rustango::server::BoxedView;
use super::views::{HomePageView, SomePageView};

pub fn get_routes() -> Vec<(&'static str, BoxedView)> {
    vec![
        ("/", Box::new(HomePageView)),
        ("/somepage", Box::new(SomePageView))
    ]
}
