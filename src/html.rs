use web_sys::wasm_bindgen::JsCast;

pub struct A(web_sys::Element);

pub fn a() -> A {
    A::new()
}

impl A {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("a").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Abbr(web_sys::Element);

pub fn abbr() -> Abbr {
    Abbr::new()
}

impl Abbr {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("abbr").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Address(web_sys::Element);

pub fn address() -> Address {
    Address::new()
}

impl Address {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("address").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Area(web_sys::Element);

pub fn area() -> Area {
    Area::new()
}

impl Area {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("area").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Article(web_sys::Element);

pub fn article() -> Article {
    Article::new()
}

impl Article {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("article").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Aside(web_sys::Element);

pub fn aside() -> Aside {
    Aside::new()
}

impl Aside {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("aside").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Audio(web_sys::Element);

pub fn audio() -> Audio {
    Audio::new()
}

impl Audio {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("audio").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn autoplay(self, value: bool) -> Self {
        self.0.set_attribute("autoplay", &value.to_string()).expect("set attribute");

        self
    }

    pub fn controls(self, value: bool) -> Self {
        self.0.set_attribute("controls", &value.to_string()).expect("set attribute");

        self
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn r#loop(self, value: bool) -> Self {
        self.0.set_attribute("loop", &value.to_string()).expect("set attribute");

        self
    }

    pub fn muted(self, value: bool) -> Self {
        self.0.set_attribute("muted", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct B(web_sys::Element);

pub fn b() -> B {
    B::new()
}

impl B {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("b").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Base(web_sys::Element);

pub fn base() -> Base {
    Base::new()
}

impl Base {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("base").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Bdi(web_sys::Element);

pub fn bdi() -> Bdi {
    Bdi::new()
}

impl Bdi {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("bdi").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Bdo(web_sys::Element);

pub fn bdo() -> Bdo {
    Bdo::new()
}

impl Bdo {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("bdo").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Blockquote(web_sys::Element);

pub fn blockquote() -> Blockquote {
    Blockquote::new()
}

impl Blockquote {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("blockquote").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Body(web_sys::Element);

pub fn body() -> Body {
    Body::new()
}

impl Body {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("body").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Br(web_sys::Element);

pub fn br() -> Br {
    Br::new()
}

impl Br {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("br").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Button(web_sys::Element);

pub fn button() -> Button {
    Button::new()
}

impl Button {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("button").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn disabled(self, value: bool) -> Self {
        self.0.set_attribute("disabled", &value.to_string()).expect("set attribute");

        self
    }

    pub fn formnovalidate(self, value: bool) -> Self {
        self.0.set_attribute("formnovalidate", &value.to_string()).expect("set attribute");

        self
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Canvas(web_sys::Element);

pub fn canvas() -> Canvas {
    Canvas::new()
}

impl Canvas {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("canvas").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Caption(web_sys::Element);

pub fn caption() -> Caption {
    Caption::new()
}

impl Caption {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("caption").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Cite(web_sys::Element);

pub fn cite() -> Cite {
    Cite::new()
}

impl Cite {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("cite").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Code(web_sys::Element);

pub fn code() -> Code {
    Code::new()
}

impl Code {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("code").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Col(web_sys::Element);

pub fn col() -> Col {
    Col::new()
}

impl Col {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("col").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Colgroup(web_sys::Element);

pub fn colgroup() -> Colgroup {
    Colgroup::new()
}

impl Colgroup {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("colgroup").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Data(web_sys::Element);

pub fn data() -> Data {
    Data::new()
}

impl Data {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("data").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Datalist(web_sys::Element);

pub fn datalist() -> Datalist {
    Datalist::new()
}

impl Datalist {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("datalist").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Dd(web_sys::Element);

pub fn dd() -> Dd {
    Dd::new()
}

impl Dd {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("dd").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Del(web_sys::Element);

pub fn del() -> Del {
    Del::new()
}

impl Del {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("del").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Details(web_sys::Element);

pub fn details() -> Details {
    Details::new()
}

impl Details {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("details").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn open(self, value: bool) -> Self {
        self.0.set_attribute("open", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Dfn(web_sys::Element);

pub fn dfn() -> Dfn {
    Dfn::new()
}

impl Dfn {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("dfn").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Dialog(web_sys::Element);

pub fn dialog() -> Dialog {
    Dialog::new()
}

impl Dialog {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("dialog").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn open(self, value: bool) -> Self {
        self.0.set_attribute("open", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Div(web_sys::Element);

pub fn div() -> Div {
    Div::new()
}

impl Div {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("div").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Dl(web_sys::Element);

pub fn dl() -> Dl {
    Dl::new()
}

impl Dl {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("dl").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Dt(web_sys::Element);

pub fn dt() -> Dt {
    Dt::new()
}

impl Dt {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("dt").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Em(web_sys::Element);

pub fn em() -> Em {
    Em::new()
}

impl Em {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("em").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Embed(web_sys::Element);

pub fn embed() -> Embed {
    Embed::new()
}

impl Embed {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("embed").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Fieldset(web_sys::Element);

pub fn fieldset() -> Fieldset {
    Fieldset::new()
}

impl Fieldset {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("fieldset").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn disabled(self, value: bool) -> Self {
        self.0.set_attribute("disabled", &value.to_string()).expect("set attribute");

        self
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Figcaption(web_sys::Element);

pub fn figcaption() -> Figcaption {
    Figcaption::new()
}

impl Figcaption {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("figcaption").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Figure(web_sys::Element);

pub fn figure() -> Figure {
    Figure::new()
}

impl Figure {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("figure").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Footer(web_sys::Element);

pub fn footer() -> Footer {
    Footer::new()
}

impl Footer {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("footer").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Form(web_sys::Element);

pub fn form() -> Form {
    Form::new()
}

impl Form {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("form").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn novalidate(self, value: bool) -> Self {
        self.0.set_attribute("novalidate", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct H1(web_sys::Element);

pub fn h1() -> H1 {
    H1::new()
}

impl H1 {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("h1").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct H2(web_sys::Element);

pub fn h2() -> H2 {
    H2::new()
}

impl H2 {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("h2").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct H3(web_sys::Element);

pub fn h3() -> H3 {
    H3::new()
}

impl H3 {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("h3").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct H4(web_sys::Element);

pub fn h4() -> H4 {
    H4::new()
}

impl H4 {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("h4").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct H5(web_sys::Element);

pub fn h5() -> H5 {
    H5::new()
}

impl H5 {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("h5").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct H6(web_sys::Element);

pub fn h6() -> H6 {
    H6::new()
}

impl H6 {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("h6").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Head(web_sys::Element);

pub fn head() -> Head {
    Head::new()
}

impl Head {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("head").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Header(web_sys::Element);

pub fn header() -> Header {
    Header::new()
}

impl Header {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("header").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Hgroup(web_sys::Element);

pub fn hgroup() -> Hgroup {
    Hgroup::new()
}

impl Hgroup {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("hgroup").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Hr(web_sys::Element);

pub fn hr() -> Hr {
    Hr::new()
}

impl Hr {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("hr").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Html(web_sys::Element);

pub fn html() -> Html {
    Html::new()
}

impl Html {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("html").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct I(web_sys::Element);

pub fn i() -> I {
    I::new()
}

impl I {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("i").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Iframe(web_sys::Element);

pub fn iframe() -> Iframe {
    Iframe::new()
}

impl Iframe {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("iframe").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn allowfullscreen(self, value: bool) -> Self {
        self.0.set_attribute("allowfullscreen", &value.to_string()).expect("set attribute");

        self
    }

    pub fn allowpaymentrequest(self, value: bool) -> Self {
        self.0.set_attribute("allowpaymentrequest", &value.to_string()).expect("set attribute");

        self
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Img(web_sys::Element);

pub fn img() -> Img {
    Img::new()
}

impl Img {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("img").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn ismap(self, value: bool) -> Self {
        self.0.set_attribute("ismap", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Input(web_sys::Element);

pub fn input() -> Input {
    Input::new()
}

impl Input {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("input").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn checked(self, value: bool) -> Self {
        self.0.set_attribute("checked", &value.to_string()).expect("set attribute");

        self
    }

    pub fn disabled(self, value: bool) -> Self {
        self.0.set_attribute("disabled", &value.to_string()).expect("set attribute");

        self
    }

    pub fn formnovalidate(self, value: bool) -> Self {
        self.0.set_attribute("formnovalidate", &value.to_string()).expect("set attribute");

        self
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn multiple(self, value: bool) -> Self {
        self.0.set_attribute("multiple", &value.to_string()).expect("set attribute");

        self
    }

    pub fn readonly(self, value: bool) -> Self {
        self.0.set_attribute("readonly", &value.to_string()).expect("set attribute");

        self
    }

    pub fn required(self, value: bool) -> Self {
        self.0.set_attribute("required", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Ins(web_sys::Element);

pub fn ins() -> Ins {
    Ins::new()
}

impl Ins {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("ins").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Kbd(web_sys::Element);

pub fn kbd() -> Kbd {
    Kbd::new()
}

impl Kbd {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("kbd").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Label(web_sys::Element);

pub fn label() -> Label {
    Label::new()
}

impl Label {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("label").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Legend(web_sys::Element);

pub fn legend() -> Legend {
    Legend::new()
}

impl Legend {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("legend").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Li(web_sys::Element);

pub fn li() -> Li {
    Li::new()
}

impl Li {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("li").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Link(web_sys::Element);

pub fn link() -> Link {
    Link::new()
}

impl Link {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("link").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Main(web_sys::Element);

pub fn main() -> Main {
    Main::new()
}

impl Main {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("main").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Map(web_sys::Element);

pub fn map() -> Map {
    Map::new()
}

impl Map {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("map").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Mark(web_sys::Element);

pub fn mark() -> Mark {
    Mark::new()
}

impl Mark {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("mark").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Math(web_sys::Element);

pub fn math() -> Math {
    Math::new()
}

impl Math {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("math").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Menu(web_sys::Element);

pub fn menu() -> Menu {
    Menu::new()
}

impl Menu {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("menu").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Meta(web_sys::Element);

pub fn meta() -> Meta {
    Meta::new()
}

impl Meta {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("meta").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Meter(web_sys::Element);

pub fn meter() -> Meter {
    Meter::new()
}

impl Meter {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("meter").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Nav(web_sys::Element);

pub fn nav() -> Nav {
    Nav::new()
}

impl Nav {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("nav").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Noscript(web_sys::Element);

pub fn noscript() -> Noscript {
    Noscript::new()
}

impl Noscript {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("noscript").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Object(web_sys::Element);

pub fn object() -> Object {
    Object::new()
}

impl Object {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("object").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Ol(web_sys::Element);

pub fn ol() -> Ol {
    Ol::new()
}

impl Ol {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("ol").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn reversed(self, value: bool) -> Self {
        self.0.set_attribute("reversed", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Optgroup(web_sys::Element);

pub fn optgroup() -> Optgroup {
    Optgroup::new()
}

impl Optgroup {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("optgroup").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn disabled(self, value: bool) -> Self {
        self.0.set_attribute("disabled", &value.to_string()).expect("set attribute");

        self
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Option(web_sys::Element);

pub fn option() -> Option {
    Option::new()
}

impl Option {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("option").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn disabled(self, value: bool) -> Self {
        self.0.set_attribute("disabled", &value.to_string()).expect("set attribute");

        self
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn selected(self, value: bool) -> Self {
        self.0.set_attribute("selected", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Output(web_sys::Element);

pub fn output() -> Output {
    Output::new()
}

impl Output {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("output").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct P(web_sys::Element);

pub fn p() -> P {
    P::new()
}

impl P {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("p").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Param(web_sys::Element);

pub fn param() -> Param {
    Param::new()
}

impl Param {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("param").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Picture(web_sys::Element);

pub fn picture() -> Picture {
    Picture::new()
}

impl Picture {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("picture").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Pre(web_sys::Element);

pub fn pre() -> Pre {
    Pre::new()
}

impl Pre {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("pre").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Progress(web_sys::Element);

pub fn progress() -> Progress {
    Progress::new()
}

impl Progress {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("progress").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Q(web_sys::Element);

pub fn q() -> Q {
    Q::new()
}

impl Q {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("q").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Rp(web_sys::Element);

pub fn rp() -> Rp {
    Rp::new()
}

impl Rp {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("rp").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Rt(web_sys::Element);

pub fn rt() -> Rt {
    Rt::new()
}

impl Rt {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("rt").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Ruby(web_sys::Element);

pub fn ruby() -> Ruby {
    Ruby::new()
}

impl Ruby {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("ruby").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct S(web_sys::Element);

pub fn s() -> S {
    S::new()
}

impl S {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("s").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Samp(web_sys::Element);

pub fn samp() -> Samp {
    Samp::new()
}

impl Samp {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("samp").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Script(web_sys::Element);

pub fn script() -> Script {
    Script::new()
}

impl Script {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("script").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn r#async(self, value: bool) -> Self {
        self.0.set_attribute("async", &value.to_string()).expect("set attribute");

        self
    }

    pub fn defer(self, value: bool) -> Self {
        self.0.set_attribute("defer", &value.to_string()).expect("set attribute");

        self
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Section(web_sys::Element);

pub fn section() -> Section {
    Section::new()
}

impl Section {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("section").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Select(web_sys::Element);

pub fn select() -> Select {
    Select::new()
}

impl Select {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("select").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn disabled(self, value: bool) -> Self {
        self.0.set_attribute("disabled", &value.to_string()).expect("set attribute");

        self
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn multiple(self, value: bool) -> Self {
        self.0.set_attribute("multiple", &value.to_string()).expect("set attribute");

        self
    }

    pub fn required(self, value: bool) -> Self {
        self.0.set_attribute("required", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Slot(web_sys::Element);

pub fn slot() -> Slot {
    Slot::new()
}

impl Slot {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("slot").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Small(web_sys::Element);

pub fn small() -> Small {
    Small::new()
}

impl Small {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("small").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Source(web_sys::Element);

pub fn source() -> Source {
    Source::new()
}

impl Source {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("source").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Span(web_sys::Element);

pub fn span() -> Span {
    Span::new()
}

impl Span {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("span").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Strong(web_sys::Element);

pub fn strong() -> Strong {
    Strong::new()
}

impl Strong {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("strong").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Style(web_sys::Element);

pub fn style() -> Style {
    Style::new()
}

impl Style {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("style").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Sub(web_sys::Element);

pub fn sub() -> Sub {
    Sub::new()
}

impl Sub {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("sub").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Summary(web_sys::Element);

pub fn summary() -> Summary {
    Summary::new()
}

impl Summary {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("summary").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Sup(web_sys::Element);

pub fn sup() -> Sup {
    Sup::new()
}

impl Sup {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("sup").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Svg(web_sys::Element);

pub fn svg() -> Svg {
    Svg::new()
}

impl Svg {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("svg").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Table(web_sys::Element);

pub fn table() -> Table {
    Table::new()
}

impl Table {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("table").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Tbody(web_sys::Element);

pub fn tbody() -> Tbody {
    Tbody::new()
}

impl Tbody {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("tbody").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Td(web_sys::Element);

pub fn td() -> Td {
    Td::new()
}

impl Td {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("td").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Template(web_sys::Element);

pub fn template() -> Template {
    Template::new()
}

impl Template {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("template").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Textarea(web_sys::Element);

pub fn textarea() -> Textarea {
    Textarea::new()
}

impl Textarea {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("textarea").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn disabled(self, value: bool) -> Self {
        self.0.set_attribute("disabled", &value.to_string()).expect("set attribute");

        self
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn readonly(self, value: bool) -> Self {
        self.0.set_attribute("readonly", &value.to_string()).expect("set attribute");

        self
    }

    pub fn required(self, value: bool) -> Self {
        self.0.set_attribute("required", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Tfoot(web_sys::Element);

pub fn tfoot() -> Tfoot {
    Tfoot::new()
}

impl Tfoot {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("tfoot").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Th(web_sys::Element);

pub fn th() -> Th {
    Th::new()
}

impl Th {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("th").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Thead(web_sys::Element);

pub fn thead() -> Thead {
    Thead::new()
}

impl Thead {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("thead").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Time(web_sys::Element);

pub fn time() -> Time {
    Time::new()
}

impl Time {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("time").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Title(web_sys::Element);

pub fn title() -> Title {
    Title::new()
}

impl Title {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("title").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Tr(web_sys::Element);

pub fn tr() -> Tr {
    Tr::new()
}

impl Tr {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("tr").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Track(web_sys::Element);

pub fn track() -> Track {
    Track::new()
}

impl Track {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("track").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn default(self, value: bool) -> Self {
        self.0.set_attribute("default", &value.to_string()).expect("set attribute");

        self
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct U(web_sys::Element);

pub fn u() -> U {
    U::new()
}

impl U {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("u").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Ul(web_sys::Element);

pub fn ul() -> Ul {
    Ul::new()
}

impl Ul {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("ul").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Var(web_sys::Element);

pub fn var() -> Var {
    Var::new()
}

impl Var {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("var").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Video(web_sys::Element);

pub fn video() -> Video {
    Video::new()
}

impl Video {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("video").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn autoplay(self, value: bool) -> Self {
        self.0.set_attribute("autoplay", &value.to_string()).expect("set attribute");

        self
    }

    pub fn controls(self, value: bool) -> Self {
        self.0.set_attribute("controls", &value.to_string()).expect("set attribute");

        self
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn r#loop(self, value: bool) -> Self {
        self.0.set_attribute("loop", &value.to_string()).expect("set attribute");

        self
    }

    pub fn muted(self, value: bool) -> Self {
        self.0.set_attribute("muted", &value.to_string()).expect("set attribute");

        self
    }

    pub fn playsinline(self, value: bool) -> Self {
        self.0.set_attribute("playsinline", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Wbr(web_sys::Element);

pub fn wbr() -> Wbr {
    Wbr::new()
}

impl Wbr {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("wbr").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }

    pub fn text(self, text: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }

    pub fn child_ui(self, ui: impl Fn(Ui)) {
        ui(Ui(self.0))
    }

    pub fn hidden(self, value: bool) -> Self {
        self.0.set_attribute("hidden", &value.to_string()).expect("set attribute");

        self
    }

    pub fn itemscope(self, value: bool) -> Self {
        self.0.set_attribute("itemscope", &value.to_string()).expect("set attribute");

        self
    }

    pub fn class(self, class: &str) -> Self {
        self.0.class_list().add_1(class).unwrap();

        self
    }
}

pub struct Ui(web_sys::Element);

impl Ui {
    pub fn a(&self) -> A {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("a").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        A(child)
    }

    pub fn abbr(&self) -> Abbr {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("abbr").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Abbr(child)
    }

    pub fn address(&self) -> Address {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("address").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Address(child)
    }

    pub fn area(&self) -> Area {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("area").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Area(child)
    }

    pub fn article(&self) -> Article {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("article").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Article(child)
    }

    pub fn aside(&self) -> Aside {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("aside").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Aside(child)
    }

    pub fn audio(&self) -> Audio {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("audio").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Audio(child)
    }

    pub fn b(&self) -> B {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("b").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        B(child)
    }

    pub fn base(&self) -> Base {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("base").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Base(child)
    }

    pub fn bdi(&self) -> Bdi {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("bdi").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Bdi(child)
    }

    pub fn bdo(&self) -> Bdo {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("bdo").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Bdo(child)
    }

    pub fn blockquote(&self) -> Blockquote {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("blockquote").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Blockquote(child)
    }

    pub fn body(&self) -> Body {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("body").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Body(child)
    }

    pub fn br(&self) -> Br {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("br").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Br(child)
    }

    pub fn button(&self) -> Button {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("button").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Button(child)
    }

    pub fn canvas(&self) -> Canvas {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("canvas").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Canvas(child)
    }

    pub fn caption(&self) -> Caption {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("caption").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Caption(child)
    }

    pub fn cite(&self) -> Cite {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("cite").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Cite(child)
    }

    pub fn code(&self) -> Code {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("code").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Code(child)
    }

    pub fn col(&self) -> Col {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("col").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Col(child)
    }

    pub fn colgroup(&self) -> Colgroup {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("colgroup").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Colgroup(child)
    }

    pub fn data(&self) -> Data {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("data").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Data(child)
    }

    pub fn datalist(&self) -> Datalist {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("datalist").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Datalist(child)
    }

    pub fn dd(&self) -> Dd {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("dd").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Dd(child)
    }

    pub fn del(&self) -> Del {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("del").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Del(child)
    }

    pub fn details(&self) -> Details {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("details").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Details(child)
    }

    pub fn dfn(&self) -> Dfn {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("dfn").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Dfn(child)
    }

    pub fn dialog(&self) -> Dialog {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("dialog").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Dialog(child)
    }

    pub fn div(&self) -> Div {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("div").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Div(child)
    }

    pub fn dl(&self) -> Dl {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("dl").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Dl(child)
    }

    pub fn dt(&self) -> Dt {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("dt").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Dt(child)
    }

    pub fn em(&self) -> Em {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("em").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Em(child)
    }

    pub fn embed(&self) -> Embed {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("embed").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Embed(child)
    }

    pub fn fieldset(&self) -> Fieldset {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("fieldset").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Fieldset(child)
    }

    pub fn figcaption(&self) -> Figcaption {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("figcaption").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Figcaption(child)
    }

    pub fn figure(&self) -> Figure {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("figure").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Figure(child)
    }

    pub fn footer(&self) -> Footer {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("footer").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Footer(child)
    }

    pub fn form(&self) -> Form {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("form").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Form(child)
    }

    pub fn h1(&self) -> H1 {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("h1").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        H1(child)
    }

    pub fn h2(&self) -> H2 {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("h2").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        H2(child)
    }

    pub fn h3(&self) -> H3 {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("h3").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        H3(child)
    }

    pub fn h4(&self) -> H4 {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("h4").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        H4(child)
    }

    pub fn h5(&self) -> H5 {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("h5").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        H5(child)
    }

    pub fn h6(&self) -> H6 {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("h6").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        H6(child)
    }

    pub fn head(&self) -> Head {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("head").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Head(child)
    }

    pub fn header(&self) -> Header {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("header").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Header(child)
    }

    pub fn hgroup(&self) -> Hgroup {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("hgroup").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Hgroup(child)
    }

    pub fn hr(&self) -> Hr {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("hr").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Hr(child)
    }

    pub fn html(&self) -> Html {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("html").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Html(child)
    }

    pub fn i(&self) -> I {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("i").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        I(child)
    }

    pub fn iframe(&self) -> Iframe {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("iframe").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Iframe(child)
    }

    pub fn img(&self) -> Img {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("img").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Img(child)
    }

    pub fn input(&self) -> Input {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("input").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Input(child)
    }

    pub fn ins(&self) -> Ins {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("ins").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Ins(child)
    }

    pub fn kbd(&self) -> Kbd {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("kbd").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Kbd(child)
    }

    pub fn label(&self) -> Label {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("label").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Label(child)
    }

    pub fn legend(&self) -> Legend {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("legend").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Legend(child)
    }

    pub fn li(&self) -> Li {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("li").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Li(child)
    }

    pub fn link(&self) -> Link {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("link").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Link(child)
    }

    pub fn main(&self) -> Main {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("main").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Main(child)
    }

    pub fn map(&self) -> Map {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("map").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Map(child)
    }

    pub fn mark(&self) -> Mark {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("mark").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Mark(child)
    }

    pub fn math(&self) -> Math {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("math").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Math(child)
    }

    pub fn menu(&self) -> Menu {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("menu").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Menu(child)
    }

    pub fn meta(&self) -> Meta {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("meta").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Meta(child)
    }

    pub fn meter(&self) -> Meter {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("meter").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Meter(child)
    }

    pub fn nav(&self) -> Nav {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("nav").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Nav(child)
    }

    pub fn noscript(&self) -> Noscript {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("noscript").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Noscript(child)
    }

    pub fn object(&self) -> Object {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("object").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Object(child)
    }

    pub fn ol(&self) -> Ol {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("ol").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Ol(child)
    }

    pub fn optgroup(&self) -> Optgroup {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("optgroup").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Optgroup(child)
    }

    pub fn option(&self) -> Option {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("option").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Option(child)
    }

    pub fn output(&self) -> Output {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("output").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Output(child)
    }

    pub fn p(&self) -> P {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("p").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        P(child)
    }

    pub fn param(&self) -> Param {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("param").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Param(child)
    }

    pub fn picture(&self) -> Picture {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("picture").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Picture(child)
    }

    pub fn pre(&self) -> Pre {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("pre").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Pre(child)
    }

    pub fn progress(&self) -> Progress {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("progress").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Progress(child)
    }

    pub fn q(&self) -> Q {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("q").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Q(child)
    }

    pub fn rp(&self) -> Rp {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("rp").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Rp(child)
    }

    pub fn rt(&self) -> Rt {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("rt").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Rt(child)
    }

    pub fn ruby(&self) -> Ruby {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("ruby").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Ruby(child)
    }

    pub fn s(&self) -> S {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("s").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        S(child)
    }

    pub fn samp(&self) -> Samp {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("samp").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Samp(child)
    }

    pub fn script(&self) -> Script {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("script").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Script(child)
    }

    pub fn section(&self) -> Section {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("section").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Section(child)
    }

    pub fn select(&self) -> Select {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("select").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Select(child)
    }

    pub fn slot(&self) -> Slot {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("slot").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Slot(child)
    }

    pub fn small(&self) -> Small {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("small").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Small(child)
    }

    pub fn source(&self) -> Source {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("source").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Source(child)
    }

    pub fn span(&self) -> Span {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("span").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Span(child)
    }

    pub fn strong(&self) -> Strong {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("strong").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Strong(child)
    }

    pub fn style(&self) -> Style {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("style").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Style(child)
    }

    pub fn sub(&self) -> Sub {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("sub").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Sub(child)
    }

    pub fn summary(&self) -> Summary {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("summary").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Summary(child)
    }

    pub fn sup(&self) -> Sup {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("sup").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Sup(child)
    }

    pub fn svg(&self) -> Svg {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("svg").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Svg(child)
    }

    pub fn table(&self) -> Table {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("table").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Table(child)
    }

    pub fn tbody(&self) -> Tbody {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("tbody").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Tbody(child)
    }

    pub fn td(&self) -> Td {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("td").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Td(child)
    }

    pub fn template(&self) -> Template {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("template").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Template(child)
    }

    pub fn textarea(&self) -> Textarea {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("textarea").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Textarea(child)
    }

    pub fn tfoot(&self) -> Tfoot {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("tfoot").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Tfoot(child)
    }

    pub fn th(&self) -> Th {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("th").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Th(child)
    }

    pub fn thead(&self) -> Thead {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("thead").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Thead(child)
    }

    pub fn time(&self) -> Time {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("time").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Time(child)
    }

    pub fn title(&self) -> Title {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("title").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Title(child)
    }

    pub fn tr(&self) -> Tr {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("tr").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Tr(child)
    }

    pub fn track(&self) -> Track {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("track").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Track(child)
    }

    pub fn u(&self) -> U {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("u").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        U(child)
    }

    pub fn ul(&self) -> Ul {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("ul").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Ul(child)
    }

    pub fn var(&self) -> Var {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("var").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Var(child)
    }

    pub fn video(&self) -> Video {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("video").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Video(child)
    }

    pub fn wbr(&self) -> Wbr {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("wbr").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        Wbr(child)
    }
}