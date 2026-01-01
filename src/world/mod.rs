pub(crate) mod light;
pub(crate) mod floor;
pub(crate) mod hud_text;
pub(crate) mod objects;

// pub mod light makes items inside of light
// accessilbe to any crates that depend on my_keyboard_project

// while pub(crate) mod light makes items inside
// of light accessilbe only to this crate (my_keyboard_project)