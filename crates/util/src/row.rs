use chrono::{DateTime, Utc};
use sqlx::{any::AnyRow, Error, Row};
use uuid::Uuid;

pub trait FromAnyRow<T>: Row {
    fn any_parse(&self, index: &str) -> Result<T, sqlx::Error>;
}

impl FromAnyRow<bool> for AnyRow {
    fn any_parse(&self, index: &str) -> Result<bool, Error> {
        self.try_get(index)
    }
}
impl FromAnyRow<Option<bool>> for AnyRow {
    fn any_parse(&self, index: &str) -> Result<Option<bool>, Error> {
        self.try_get(index)
    }
}

impl FromAnyRow<String> for AnyRow {
    fn any_parse(&self, index: &str) -> Result<String, sqlx::Error> {
        self.try_get(index)
    }
}

impl FromAnyRow<Option<String>> for AnyRow {
    fn any_parse(&self, index: &str) -> Result<Option<String>, Error> {
        let res: Option<String> = self.try_get(index)?;
        Ok(res)
    }
}
impl FromAnyRow<Uuid> for AnyRow {
    fn any_parse(&self, index: &str) -> Result<Uuid, sqlx::Error> {
        let res: &str = self.try_get(index)?;
        let res = Uuid::parse_str(res).map_err(|e| sqlx::Error::ColumnDecode {
            index: index.into(),
            source: Box::new(e),
        })?;
        Ok(res)
    }
}

impl FromAnyRow<Option<Uuid>> for AnyRow {
    fn any_parse(&self, index: &str) -> Result<Option<Uuid>, sqlx::Error> {
        let res: Option<&str> = self.try_get(index)?;
        let res = if let Some(res) = res {
            Some(Uuid::parse_str(res).map_err(|e| sqlx::Error::ColumnDecode {
                index: index.into(),
                source: Box::new(e),
            })?)
        } else {
            None
        };
        Ok(res)
    }
}

impl FromAnyRow<DateTime<Utc>> for AnyRow {
    fn any_parse(&self, index: &str) -> Result<DateTime<Utc>, Error> {
        let res: &str = self.try_get(index)?;
        let res = DateTime::parse_from_rfc3339(res)
            .map_err(|e| Error::ColumnDecode {
                index: index.into(),
                source: Box::new(e),
            })?
            .to_utc();

        Ok(res)
    }
}

impl FromAnyRow<serde_json::Value> for AnyRow {
    fn any_parse(&self, index: &str) -> Result<serde_json::Value, sqlx::Error> {
        let res: &str = self.try_get(index)?;
        let res: serde_json::Value =
            serde_json::from_str(res).map_err(|e| Error::ColumnDecode {
                index: index.into(),
                source: Box::new(e),
            })?;
        Ok(res)
    }
}

impl FromAnyRow<Option<serde_json::Value>> for AnyRow {
    fn any_parse(&self, index: &str) -> Result<Option<serde_json::Value>, sqlx::Error> {
        let res: Option<&str> = self.try_get(index)?;
        let res = if let Some(res) = res {
            let res: serde_json::Value = self.any_parse(index)?;
            Some(res)
        } else {
            None
        };
        Ok(res)
    }
}
