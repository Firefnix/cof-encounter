use colored::{ColoredString, Colorize};

use crate::db::*;

const TEXT_BOX_SIZE: usize = 12;

pub trait Titled {
    fn title(&self) -> &str;
}

impl Titled for Npc {
    fn title(&self) -> &str {
        self.name.as_str()
    }
}

impl Titled for Artifact {
    fn title(&self) -> &str {
        self.name.as_str()
    }
}

impl Titled for Place {
    fn title(&self) -> &str {
        self.name.as_str()
    }
}

pub fn display_in_text_box<T: ToString + Titled>(item: &T) {
    println!("{}", wrap_in_text_box(item.title(), &item.to_string()));
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        format!("#{}", self.name.to_lowercase())
    }
}

impl ToString for Illustration {
    fn to_string(&self) -> String {
        format!(
            "{} {}",
            self.path,
            match (self.width, self.height) {
                (Some(w), Some(h)) => format!("{}x{}", w, h).green(),
                _ => ColoredString::from(""),
            }
        )
    }
}

impl ToString for Npc {
    fn to_string(&self) -> String {
        numeric_field("for", self.force)
            + &numeric_field("dex", self.dex)
            + &numeric_field("con", self.con)
            + &numeric_field("int", self.int)
            + &numeric_field("sag", self.sag)
            + &numeric_field("cha", self.cha)
            + &numeric_field("def", self.def)
            + &numeric_field("pv", self.pv)
            + &numeric_field("init", self.init)
            + &(match &self.illustration {
                Some(illustration) => illustration.to_string(),
                _ => String::from(""),
            })
    }
}

impl ToString for Artifact {
    fn to_string(&self) -> String {
        let price = match self.price {
            Some(price) => format_price(price),
            _ => ColoredString::from(""),
        };
        format!("{} pc\n", price)
    }
}

fn format_price(price: SqlInteger) -> ColoredString {
    if price < 0 {
        price.to_string().red().bold()
    } else if price == 0 {
        price.to_string().magenta().bold()
    } else if price < 10 {
        price.to_string().green()
    } else if price < 100 {
        price.to_string().blue()
    } else if price < 1000 {
        price.to_string().yellow()
    } else {
        price.to_string().red()
    }
}

impl ToString for Place {
    fn to_string(&self) -> String {
        format!("{} PNJs\n{} artefacts", self.npcs.len(), self.artifacts.len())
    }
}

fn numeric_field(field: &str, value: Option<SqlInteger>) -> String {
    match value {
        Some(value) => {
            let padding_right =
                (" ".repeat(TEXT_BOX_SIZE - 5 - field.len() - value.to_string().len()) + "|")
                    .bold();
            format!(
                "{} {} {} {}",
                "|".bold(),
                field.to_uppercase(),
                field_value_to_string(value),
                padding_right
            )
        }
        _ => String::from(""),
    }
}

fn wrap_line(line: &str) -> String {
    let padding_right = (" ".repeat(TEXT_BOX_SIZE - 4 - line.len()) + "|").bold();
    format!("{} {} {}", "|".bold(), line, padding_right)
}

fn wrap_in_text_box(title: &str, content: &str) -> String {
    let padding_top_left: ColoredString = "-".repeat((TEXT_BOX_SIZE - title.len() - 2) / 2).bold();
    let padding_top_right = "-".repeat((TEXT_BOX_SIZE - title.len() - 1) / 2).bold();
    let padding_bottom = "-".repeat(TEXT_BOX_SIZE).bold();
    format!(
        "{} {} {}",
        padding_top_left,
        title.bold().purple(),
        padding_top_right
    ) + content
        .split("\n")
        .map(wrap_line)
        .collect::<Vec<String>>()
        .join("\n")
        .as_str()
        + &format!("{}", padding_bottom)
}

fn field_value_to_string(value: SqlInteger) -> ColoredString {
    if value < -1 {
        value.to_string().red().bold()
    } else if value < 0 {
        value.to_string().red()
    } else if value == 0 || value == 1 {
        value.to_string().blue()
    } else if value < 2 {
        value.to_string().green()
    } else {
        value.to_string().green().bold()
    }
}
