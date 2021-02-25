use lazy_static::lazy_static;
use syntect::{dumps::from_binary, html::css_for_theme_with_class_style};
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
    css_for_theme_with_class_style(&theme,
        syntect::html::ClassStyle::Spaced)
}

/// Returns the highlighter and whether it was found in the extra or not
pub fn get_highlighter(lang: &str, config: &Config) -> (SyntaxReference, bool) {
    let mut in_extra = false;

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
}
