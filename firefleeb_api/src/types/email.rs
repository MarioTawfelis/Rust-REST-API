use serde::{Deserialize, Serialize};
use regex::Regex;


use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::pg::{Pg, PgValue};
use diesel::{FromSqlRow, AsExpression};
use diesel::sql_types::Text;
use std::io::Write;


#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
pub struct Email(String);

impl Email {
    pub fn parse(s: &str) -> Result<Self, String> {
        let email_re = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
        if email_re.is_match(s) {
            Ok(Self(s.to_lowercase()))
        } else {
            Err(format!("Invalid email format: {}", s))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ToSql<Text, Pg> for Email {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.0.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for Email {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes())?;
        Email::parse(s).map_err(|e| e.into())
    }
}

