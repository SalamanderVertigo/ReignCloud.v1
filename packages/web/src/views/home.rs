use dioxus::prelude::*;
use ui::{AuthTest, Echo, Hero};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
        AuthTest {}
    }
}
