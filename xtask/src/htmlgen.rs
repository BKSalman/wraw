use anyhow::Context;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::flags::Htmlgen;

const RUST_KEYWORDS: &[&str] = &["loop", "async"];

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
struct HTMLElementsSpec {
    __META__: serde_json::Value,
    #[serde(flatten)]
    elements: IndexMap<String, HTMLElement>,
}

#[derive(Debug, Deserialize, Serialize)]
struct HTMLElement {
    attributes: Vec<String>,
    categories: Vec<String>,
    children: Vec<String>,
    desc: String,
}

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
struct HTMLAttributesSpec {
    __META__: serde_json::Value,
    #[serde(flatten)]
    attributes: IndexMap<String, HTMLAttribute>,
}

#[derive(Debug, Deserialize, Serialize)]
struct HTMLAttribute {
    desc: String,
    elements: Vec<String>,
    value_keywords: Vec<String>,
    value_type: String,
}

pub fn generate(htmlgen: Htmlgen) -> anyhow::Result<()> {
    let elements_spec_json = std::fs::read_to_string(htmlgen.path.join("elements.json"))
        .context("Read HTML spec json file")?;
    let attributes_spec_json = std::fs::read_to_string(htmlgen.path.join("attributes.json"))
        .context("Read HTML spec json file")?;

    let elements_spec = serde_json::from_str::<HTMLElementsSpec>(&elements_spec_json)?;
    let attributes_spec = serde_json::from_str::<HTMLAttributesSpec>(&attributes_spec_json)?;

    let mut writer: Box<dyn std::io::Write> = if htmlgen.stdout || htmlgen.output_path.is_none() {
        Box::new(std::io::stdout())
    } else {
        let output_path = htmlgen.output_path.unwrap();
        let output_path = if output_path.is_dir() {
            output_path.join("html.rs")
        } else {
            output_path.with_file_name("html.rs")
        };
        Box::new(
            std::fs::File::create(&output_path)
                .with_context(|| format!("open {}", output_path.display()))?,
        )
    };

    writeln!(
        &mut writer,
        r#"use web_sys::wasm_bindgen::JsCast;
"#
    )
    .with_context(|| "writing imports")?;

    for (element_name, element_info) in &elements_spec.elements {
        let element_type_name = uppercase_first_letter(&element_name);
        write!(
            &mut writer,
            r#"pub struct {element_type_name}(web_sys::Element);

pub fn {element_name}() -> {element_type_name} {{
    {element_type_name}::new()
}}

impl {element_type_name} {{
    pub fn new() -> Self {{
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let element = document.create_element("{element_name}").unwrap();
        body.append_child(&element).unwrap();

        Self(element)
    }}

    pub fn text(self, text: &str) -> Self {{
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let text_node = document.create_text_node(text);

        self.0.append_child(&text_node).unwrap();

        self
    }}

    pub fn child_ui(self, ui: impl Fn(Ui)) {{
        ui(Ui(self.0))
    }}
"#
        )
        .with_context(|| format!("writing struct for element {element_name}"))?;

        for attr in &element_info.attributes {
            if let Some(attr_spec) = attributes_spec.attributes.get(attr) {
                if attr_spec.value_type == "Boolean attribute" {
                    let attr_fn = if RUST_KEYWORDS.contains(&attr.as_str()) {
                        format!("r#{attr}")
                    } else {
                        attr.to_string()
                    };

                    write!(
                        &mut writer,
                        r#"
    pub fn {attr_fn}(self, value: bool) -> Self {{
        self.0.set_attribute("{attr}", &value.to_string()).expect("set attribute");

        self
    }}
"#
                    )
                    .with_context(|| "writing {attr} function")?;
                }
            }
        }

        write!(
            &mut writer,
            r#"
    pub fn class(self, class: &str) -> Self {{
        self.0.class_list().add_1(class).unwrap();

        self
    }}
"#
        )
        .with_context(|| "writing class function")?;

        writeln!(&mut writer, "}}\n")
            .with_context(|| format!("closing {element_name} struct impl"))?;
    }

    writeln!(&mut writer, "pub struct Ui(web_sys::Element);\n")
        .with_context(|| "writing Ui struct")?;

    write!(&mut writer, "impl Ui {{").with_context(|| "starting Ui impl")?;

    for (element_name, _element_info) in &elements_spec.elements {
        let element_type_name = uppercase_first_letter(&element_name);
        write!(
            &mut writer,
            r#"
    pub fn {element_name}(&self) -> {element_type_name} {{
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let child = document.create_element("{element_name}").unwrap();

        self.0.append_child(child.unchecked_ref()).unwrap();

        {element_type_name}(child)
    }}
"#
        )
        .with_context(|| format!("writing {element_name} function in Ui impl"))?;
    }

    write!(&mut writer, "}}").with_context(|| "closing Ui impl")?;

    Ok(())
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
