pub mod answer;
pub mod external;
pub mod form;
pub mod question;
pub mod session;

mod macros;
pub mod shared;
pub use shared::D1EntityQueries;

use serde::{de, Deserialize, Deserializer};
use time::OffsetDateTime;

pub fn date_deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let date = i64::deserialize(deserializer)?;
    OffsetDateTime::from_unix_timestamp(date)
        .map_err(|err| de::Error::invalid_value(de::Unexpected::Signed(date), &err))
}
