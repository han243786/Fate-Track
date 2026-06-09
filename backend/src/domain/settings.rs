#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPreference {
    pub default_calendar: String,
    pub privacy_default: String,
    pub language: String,
    pub theme: String,
    pub show_professional_fields: bool,
    pub show_nayin: bool,
    pub show_void_branches: bool,
    pub show_shensha: bool,
}

impl UserPreference {
    pub fn update(
        &mut self,
        default_calendar: Option<String>,
        privacy_default: Option<String>,
        language: Option<String>,
        theme: Option<String>,
    ) -> Result<(), String> {
        if let Some(default_calendar) = default_calendar {
            validate_one("default_calendar", &default_calendar, &["gregorian"])?;
            self.default_calendar = default_calendar;
        }
        if let Some(privacy_default) = privacy_default {
            validate_one(
                "privacy_default",
                &privacy_default,
                &["private", "shared_snapshot"],
            )?;
            self.privacy_default = privacy_default;
        }
        if let Some(language) = language {
            validate_one("language", &language, &["zh-CN", "en-US"])?;
            self.language = language;
        }
        if let Some(theme) = theme {
            validate_one("theme", &theme, &["system", "light", "dark"])?;
            self.theme = theme;
        }

        Ok(())
    }
}

impl Default for UserPreference {
    fn default() -> Self {
        Self {
            default_calendar: "gregorian".to_string(),
            privacy_default: "private".to_string(),
            language: "zh-CN".to_string(),
            theme: "system".to_string(),
            show_professional_fields: false,
            show_nayin: false,
            show_void_branches: false,
            show_shensha: false,
        }
    }
}

fn validate_one(field: &str, value: &str, allowed: &[&str]) -> Result<(), String> {
    if allowed.contains(&value) {
        Ok(())
    } else {
        Err(format!("unsupported {field} value: {value}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_local_private_zh_preferences() {
        let preference = UserPreference::default();

        assert_eq!(preference.default_calendar, "gregorian");
        assert_eq!(preference.privacy_default, "private");
        assert_eq!(preference.language, "zh-CN");
        assert_eq!(preference.theme, "system");
        assert!(!preference.show_professional_fields);
    }

    #[test]
    fn rejects_unsupported_calendar() {
        let mut preference = UserPreference::default();
        let error = preference
            .update(Some("lunar".to_string()), None, None, None)
            .unwrap_err();

        assert_eq!(error, "unsupported default_calendar value: lunar");
    }
}
