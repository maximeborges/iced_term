pub mod actions;
mod backend;
pub mod bindings;
mod font;
mod term;
mod theme;
mod view;

pub use alacritty_terminal::{event::Event as AlacrittyEvent, term::TermMode};
pub use backend::{settings::BackendSettings, BackendCommand};
pub use font::FontSettings;
pub use term::{Command, Event, Term, TermSettings, ViewProxy};
pub use theme::{ColorPalette, TermTheme};
pub use view::{term_view, TermView, TermViewState};
