use web_sys::{Document, Element};

struct El(web_sys::Element);

impl El {
    fn new(tag: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        Self(document.create_element(tag).unwrap())
    }

    fn child_ui(self, ui: impl Fn(Document, El)) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        ui(document, self)
    }
}

impl std::ops::Deref for El {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
