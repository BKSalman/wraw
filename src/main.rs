pub mod html;

use html::*;

fn main() {
    div().text("lmao").child_ui(|ui| {
        ui.div()
            .class("something")
            .class("wow")
            .text("lmfao")
            .child_ui(|ui| {
                ui.button().text("xqcL");
            });
        ui.button().text("WOW");
        ui.video().r#loop().text("lmao").src("");
    });
}
