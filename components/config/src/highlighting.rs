use lazy_static::lazy_static;
use syntect::dumps::from_binary;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::parsing::SyntaxReference;

use crate::config::Config;

lazy_static! {
    pub static ref SYNTAX_SET: SyntaxSet = {
        let ss: SyntaxSet =
            from_binary(include_bytes!("../../../sublime/syntaxes/newlines.packdump"));
        ss
    };
    pub static ref THEME_SET: ThemeSet =
        from_binary(include_bytes!("../../../sublime/themes/all.themedump"));
}

pub fn get_css(theme: &syntect::highlighting::Theme) -> String {
    syntect::html::css_for_theme(&theme)
}

/// Returns the highlighter and whether it was found in the extra or not
pub fn get_highlighter(info: &str, config: &Config) -> (SyntaxReference, bool) {
    let mut in_extra = false;

    if let Some(ref lang) = info.split(' ').next() {
        let syntax = SYNTAX_SET
            .find_syntax_by_token(lang)
            .or_else(|| {
                if let Some(ref extra) = config.extra_syntax_set {
                    let s = extra.find_syntax_by_token(lang);
                    if s.is_some() {
                        in_extra = true;
                    }
                    s
                } else {
                    None
                }
            })
            .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
        (syntax.clone(), in_extra)
    } else {
        (SYNTAX_SET.find_syntax_plain_text().clone(), false)
    }
}
