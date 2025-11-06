use once_cell::sync::Lazy;
use regex::Regex;

pub static RE_EMOTE: Lazy<Regex> = Lazy::new(|| Regex::new(r"<.*?:\w*:\d*>").unwrap());
pub static RE_EMOTE_TYPED: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\B(:|;|\.)[a-zA-Z0-9_-]+(:|;|\.)\B)").unwrap());
pub static RE_EMOTE_CLEAN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(:|;|\.)").unwrap());
