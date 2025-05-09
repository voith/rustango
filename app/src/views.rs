use rustango::server::{Response, View};

pub struct HomePageView;
pub struct SomePageView;

impl View for HomePageView {
    fn get(&self) -> Response {
        let html = r#"
            <!DOCTYPE html>
            <html lang="en">
                <head>
                <meta charset="utf-8">
                <title>Hello!</title>
                </head>
                <body>
                <h1>Hello!</h1>
                <p>This is the Home Page</p>
                </body>
            </html>
        "#;
        Response {
            data: html.to_string(),
            status_code:200,
            version: "1.1"
        }
    }
}


impl View for SomePageView {
    fn get(&self) -> Response {
        let html = r#"
            <!DOCTYPE html>
            <html lang="en">
                <head>
                <meta charset="utf-8">
                <title>Some Page</title>
                </head>
                <body>
                <h1>Hey!</h1>
                <p>This is the Some Page</p>
                </body>
            </html>
        "#;
        Response {
            data: html.to_string(),
            status_code:200,
            version: "1.1"
        }
    }
}
