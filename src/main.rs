mod markdown;
mod window;

use std::sync::{Mutex, OnceLock};

use cosmic::app::Settings;
use cosmic_text::{FontSystem, SwashCache, SyntaxSystem};
use window::Window;

static SYNTAX_SYSTEM: OnceLock<SyntaxSystem> = OnceLock::new();
static SWASH_CACHE: OnceLock<Mutex<SwashCache>> = OnceLock::new();
static FONT_SYSTEM: OnceLock<Mutex<FontSystem>> = OnceLock::new();

fn main() -> cosmic::iced::Result {
    let settings = Settings::default();

    SYNTAX_SYSTEM.get_or_init(SyntaxSystem::new);
    SWASH_CACHE.get_or_init(|| Mutex::new(SwashCache::new()));
    FONT_SYSTEM.get_or_init(|| Mutex::new(FontSystem::new()));

    cosmic::app::run::<Window>(settings, ())
}
