use reedline::{Reedline, Signal};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}")
    }
}

fn run() -> std::io::Result<()> {
    let mut line_editor = Reedline::create()?.with_edit_mode(Box::new(reedline::Vi::default()));
    let prompt = ClcPrompt::new();
    let calculator = calculator::Calculator::new();

    loop {
        let sig = line_editor.read_line(&prompt)?;
        match sig {
            Signal::Success(line) => match calculator.calculate_line(&line) {
                Ok(eval) => println!("{eval}"),
                Err(err) => eprintln!("{err}"),
            },
            Signal::CtrlD | Signal::CtrlC => {
                println!("bye");
                break;
            }
            Signal::CtrlL => {
                line_editor.clear_screen()?;
            }
        }
    }

    Ok(())
}

use reedline::{PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus, PromptViMode};
use std::borrow::Cow;

struct ClcPrompt;

impl ClcPrompt {
    pub const DEFAULT_PROMPT_INDICATOR: &'static str = "❯ ";
    pub const DEFAULT_VI_INSERT_PROMPT_INDICATOR: &'static str = "❯ ";
    pub const DEFAULT_VI_NORMAL_PROMPT_INDICATOR: &'static str = ": ";
    pub const DEFAULT_MULTILINE_INDICATOR: &'static str = "::: ";

    fn new() -> Self {
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
