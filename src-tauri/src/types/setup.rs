use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SetupOptions {
    wine_prefix: String,
    // desktop_shortcut_checked: bool,
    // menu_shortcut_checked: bool
}

impl SetupOptions {
    pub fn wine_prefix(&self) -> &str {
        &self.wine_prefix
    }
}
