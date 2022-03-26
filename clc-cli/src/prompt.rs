use std::borrow::Cow;

use reedline::{PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus, PromptViMode};

pub struct ClcPrompt;

impl ClcPrompt {
    pub const DEFAULT_PROMPT_INDICATOR: &'static str = "❯ ";
    pub const DEFAULT_VI_INSERT_PROMPT_INDICATOR: &'static str = "❯ ";
    pub const DEFAULT_VI_NORMAL_PROMPT_INDICATOR: &'static str = ": ";
    pub const DEFAULT_MULTILINE_INDICATOR: &'static str = "::: ";

    pub fn new() -> Self {
        Self
    }
}

impl reedline::Prompt for ClcPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        {
            Cow::Owned("".to_owned())
        }
    }
    fn render_prompt_right(&self) -> Cow<str> {
        {
            Cow::Borrowed("")
        }
    }

    fn render_prompt_indicator(&self, edit_mode: PromptEditMode) -> Cow<str> {
        match edit_mode {
            PromptEditMode::Default | PromptEditMode::Emacs => {
                Self::DEFAULT_PROMPT_INDICATOR.into()
            }
            PromptEditMode::Vi(vi_mode) => match vi_mode {
                PromptViMode::Normal => Self::DEFAULT_VI_NORMAL_PROMPT_INDICATOR.into(),
                PromptViMode::Insert => Self::DEFAULT_VI_INSERT_PROMPT_INDICATOR.into(),
            },
            PromptEditMode::Custom(str) => format!("({})", str).into(),
        }
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        Cow::Borrowed(Self::DEFAULT_MULTILINE_INDICATOR)
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: PromptHistorySearch,
    ) -> Cow<str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };
        // NOTE: magic strings, given there is logic on how these compose I am not sure if it
        // is worth extracting in to static constant
        Cow::Owned(format!(
            "({}reverse-search: {}) ",
            prefix, history_search.term
        ))
    }
}

impl Default for ClcPrompt {
    fn default() -> Self {
        ClcPrompt::new()
    }
}
