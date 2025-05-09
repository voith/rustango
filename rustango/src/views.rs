use super::http::Response;

pub trait View {
    fn get(&self) -> Response {
        self.default_response()
    }

    fn post(&self) -> Response {
        self.default_response()
    }

    fn put(&self) -> Response {
        self.default_response()
    }

    fn patch(&self) -> Response {
        self.default_response()
    }

    fn delete(&self) -> Response {
        self.default_response()
    }

    fn head(&self) -> Response {
        self.default_response()
    }

    fn options<'a>(&self) -> Response {
        self.default_response()
    }

    fn default_response(&self) -> Response {
        Response {
            data: "<!DOCTYPE html><html lang='en'>405 Method Not Allowed</html>".to_string(),
            status_code: 405,
            version: "1.1",
        }
    }
}

pub type BoxedView = Box<dyn View + Send + Sync>;
