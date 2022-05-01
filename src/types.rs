use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) enum Directive {
    Note,
    Abstract,
    Info,
    Tip,
    Success,
    Question,
    Warning,
    Failure,
    Danger,
    Bug,
    Example,
    Quote,
}

impl FromStr for Directive {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "note" => Ok(Self::Note),
            "abstract" | "summary" | "tldr" => Ok(Self::Abstract),
            "info" | "todo" => Ok(Self::Info),
            "tip" | "hint" | "important" => Ok(Self::Tip),
            "success" | "check" | "done" => Ok(Self::Success),
            "question" | "help" | "faq" => Ok(Self::Question),
            "warning" | "caution" | "attention" => Ok(Self::Warning),
            "failure" | "fail" | "missing" => Ok(Self::Failure),
            "danger" | "error" => Ok(Self::Danger),
            "bug" => Ok(Self::Bug),
            "example" => Ok(Self::Example),
            "quote" | "cite" => Ok(Self::Quote),
            _ => Err(()),
        }
    }
}
