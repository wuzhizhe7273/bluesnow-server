use std::fmt::Formatter;

pub struct CdcConfig {}

#[derive(Clone)]
pub struct Field {
    name: String,
    ty: String,
    if_null: bool,
}

pub struct Fields(Vec<Field>);
impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let if_null = if self.if_null { "NULL" } else { "NOT NULL" };
        write!(f, "{} {} {}", self.name, self.ty.to_uppercase(), if_null)
    }
}

impl std::fmt::Display for Fields {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = self
            .0
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{}", result)
    }
}

#[derive(strum::EnumString, strum::Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Event {
    Insert,
    Delete,
    Update,
}
fn build_create_trigger_query(trigger: &str, table: &str, event: Event,fields:Fields,prefix:&str,log_table:&str) -> String {
    let columns=fields.0.clone().into_iter().map(|item| item.name).collect::<Vec<_>>().join(", ");
    let values=fields.0.into_iter().map(|item|format!("new.{}",item.name)).collect::<Vec<_>>().join(", ");
    format!(
        r#"
        CREATE TRIGGER `{trigger}` AFTER `{event}`
        ON `{table}`
            INSERT INTO `{log_table}` ({columns}) VALUES ({values});
        BEGIN
        END;
    "#
    )
}

fn build_create_log_table_query(prefix: &str, log_table: &str, fields: Fields) -> String {
    format!(
        r#"
        CREATE TABLE IF NOT EXISTS `{prefix}{log_table}` (
            `{prefix}_id` INTEGER PRIMARY KEY AUTOINCREMENT,
            {fields}
        );
    "#
    )
}
