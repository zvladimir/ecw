//! Texts of a calculator's reply. The backend knows no languages: a text is either ready as it is
//! (numbers, symbols, names of quantities) or a message, an id with arguments, that the UI
//! renders in the language of the interface (`ui/locales/en.js` has the English texts).

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;

use serde::Serialize;

/// A text of a reply: ready to show, or a message for the UI to translate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum Text {
    /// Shown as it is.
    Plain(String),
    /// `{ "id": …, "args": { name: text, … } }`; the arguments may be messages too.
    Message {
        /// The id of the message in the locale dictionary, like `error.solve.ordered`.
        id: Cow<'static, str>,
        /// The arguments, by name; they are inserted into the translated text.
        #[serde(skip_serializing_if = "BTreeMap::is_empty")]
        args: BTreeMap<&'static str, Text>,
    },
}

impl Text {
    /// A message without arguments.
    pub fn message(id: impl Into<Cow<'static, str>>) -> Self {
        Self::Message {
            id: id.into(),
            args: BTreeMap::new(),
        }
    }

    /// Adds the argument `name` to a message; a plain text has no place for it.
    pub fn with(mut self, name: &'static str, value: impl Into<Text>) -> Self {
        match &mut self {
            Self::Message { args, .. } => {
                args.insert(name, value.into());
            }
            Self::Plain(_) => panic!("a plain text takes no arguments"),
        }
        self
    }

    /// The id of a message.
    pub fn id(&self) -> Option<&str> {
        match self {
            Self::Message { id, .. } => Some(id),
            Self::Plain(_) => None,
        }
    }

    /// The argument `name` of a message.
    pub fn arg(&self, name: &str) -> Option<&Text> {
        match self {
            Self::Message { args, .. } => args.get(name),
            Self::Plain(_) => None,
        }
    }

    /// True for an empty plain text.
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Plain(s) if s.is_empty())
    }
}

/// `id` or `id(name=…, …)` for a message: for logs and tests, not for the screen.
impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plain(s) => f.write_str(s),
            Self::Message { id, args } if args.is_empty() => f.write_str(id),
            Self::Message { id, args } => {
                write!(f, "{id}(")?;
                for (i, (name, value)) in args.iter().enumerate() {
                    write!(f, "{}{name}={value}", if i == 0 { "" } else { ", " })?;
                }
                f.write_str(")")
            }
        }
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::Plain(String::new())
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self::Plain(s)
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self::Plain(s.to_owned())
    }
}

impl From<&String> for Text {
    fn from(s: &String) -> Self {
        Self::Plain(s.clone())
    }
}

impl From<Cow<'_, str>> for Text {
    fn from(s: Cow<'_, str>) -> Self {
        Self::Plain(s.into_owned())
    }
}

impl From<&Text> for Text {
    fn from(t: &Text) -> Self {
        t.clone()
    }
}

macro_rules! from_number {
    ($($t:ty),*) => {$(
        impl From<$t> for Text {
            fn from(n: $t) -> Self {
                Self::Plain(n.to_string())
            }
        }
    )*};
}
from_number!(usize, u32, i32, f64);

impl PartialEq<&str> for Text {
    fn eq(&self, other: &&str) -> bool {
        matches!(self, Self::Plain(s) if s == other)
    }
}

/// A message: `msg!("error.solve.ordered", left = a, right = b)`. The arguments are anything
/// that becomes a [`Text`]: strings, numbers, other messages.
#[macro_export]
macro_rules! msg {
    ($id:expr) => {
        $crate::domain::Text::message($id)
    };
    ($id:expr, $($name:ident = $value:expr),+ $(,)?) => {
        $crate::domain::Text::message($id)$(.with(stringify!($name), $value))+
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_is_a_string_and_a_message_is_an_object() {
        assert_eq!(
            serde_json::to_string(&Text::from("1.5 V")).unwrap(),
            r#""1.5 V""#
        );
        let m = msg!("a.b", x = 3usize, y = msg!("c"));
        assert_eq!(
            serde_json::to_string(&m).unwrap(),
            r#"{"id":"a.b","args":{"x":"3","y":{"id":"c"}}}"#
        );
        assert_eq!(m.to_string(), "a.b(x=3, y=c)");
    }
}
