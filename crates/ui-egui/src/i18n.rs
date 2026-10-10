//! Interface language, scoped to each egui context. Workbook data and command IDs stay intact.
use egui::{Context, Ui};
use serde::{Deserialize, Serialize};

/// Supported interface languages. Unknown saved values fall back to English.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "zh-CN")]
    Chinese,
    #[default]
    #[serde(rename = "en", other)]
    English,
}

include!(concat!(env!("OUT_DIR"), "/translations.rs"));

impl Language {
    pub const ALL: [Self; 2] = [Self::English, Self::Chinese];

    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Chinese => "zh-CN",
        }
    }

    /// Language names use their own script so either language can always be found.
    pub fn name(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Chinese => "简体中文",
        }
    }

    /// Strict command parsing: invalid codes return an error without changing preferences.
    pub fn parse(code: &str) -> Result<Self, String> {
        match code {
            "en" => Ok(Self::English),
            "zh-CN" => Ok(Self::Chinese),
            _ => Err("language must be en or zh-CN".into()),
        }
    }

    /// Translate UI text, preserving shortcut suffixes, punctuation and unknown English text.
    /// Only call this for interface strings, never workbook values, names or user input.
    pub fn text(self, source: &str) -> String {
        if self == Self::English {
            return source.into();
        }
        let normalized = if source.contains('\n') { std::borrow::Cow::Owned(source.replace('\n', " ")) } else { std::borrow::Cow::Borrowed(source) };
        let trimmed = normalized.trim();
        if let Some(value) = chinese(trimmed) {
            return value.into();
        }
        for suffix in ["…", "...", ":"] {
            if let Some(base) = trimmed.strip_suffix(suffix).and_then(chinese) {
                return format!("{base}{suffix}");
            }
        }
        if let Some((base, suffix)) = trimmed.rsplit_once(" (")
            && trimmed.ends_with(')')
            && let Some(value) = chinese(base)
        {
            return format!("{value} ({suffix}");
        }
        source.into()
    }

    /// Substitute named fields in a translated template. Values are copied verbatim in a
    /// single pass, so workbook strings containing braces cannot become new placeholders.
    pub fn format(self, source: &str, values: &[(&str, &str)]) -> String {
        let template = self.text(source);
        let mut rest = template.as_str();
        let mut result = String::new();
        while let Some((prefix, tail)) = rest.split_once('{') {
            result.push_str(prefix);
            let Some((key, after)) = tail.split_once('}') else {
                result.push('{');
                result.push_str(tail);
                return result;
            };
            if let Some((_, value)) = values.iter().find(|(name, _)| *name == key) {
                result.push_str(value);
            } else {
                result.push('{');
                result.push_str(key);
                result.push('}');
            }
            rest = after;
        }
        result.push_str(rest);
        result
    }

    /// Publish the current app's language to rendering helpers; no process-global locale.
    pub fn install(self, ctx: &Context) {
        ctx.data_mut(|data| data.insert_temp(egui::Id::new("gridfabrica.language"), self));
    }

    pub fn current(ctx: &Context) -> Self {
        ctx.data(|data| data.get_temp(egui::Id::new("gridfabrica.language"))).unwrap_or_default()
    }
}

/// Translate interface text using this widget's context.
pub fn tr(ui: &Ui, source: &str) -> String {
    Language::current(ui.ctx()).text(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translations_preserve_identifiers_and_shortcuts() {
        assert_eq!(Language::English.text("Home"), "Home");
        assert_eq!(Language::Chinese.text("Home"), "开始");
        assert_eq!(Language::Chinese.text("Save (⌘S)"), "保存 (⌘S)");
        assert_eq!(Language::Chinese.text("Freeze\nPanes"), "冻结窗格");
        assert_eq!(Language::Chinese.text("Open…"), "打开…");
        assert_eq!(Language::Chinese.text("=SUM(A1:A9)"), "=SUM(A1:A9)");
        assert_eq!(Language::Chinese.text("cell.set"), "cell.set");
    }

    #[test]
    fn template_values_are_never_translated_or_expanded() {
        assert_eq!(
            Language::Chinese.format("Save changes to {title}?", &[("title", "Home {other}"), ("other", "wrong")]),
            "保存对 Home {other} 的更改？"
        );
    }

    #[test]
    fn contexts_have_independent_languages() {
        let chinese_context = Context::default();
        let english_context = Context::default();
        Language::Chinese.install(&chinese_context);
        assert_eq!(Language::current(&chinese_context), Language::Chinese);
        assert_eq!(Language::current(&english_context), Language::English);
    }

    #[test]
    fn unknown_saved_language_falls_back_but_commands_reject_it() {
        assert_eq!(serde_json::from_str::<Language>("\"future\"").unwrap(), Language::English);
        assert!(Language::parse("future").is_err());
    }
}
