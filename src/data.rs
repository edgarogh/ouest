use chrono::NaiveDate;
use crate::OuestError;
use std::collections::HashMap;
use std::cmp::Ordering;
use serde::de::Unexpected;

#[derive(serde::Deserialize)]
pub struct Data<'a> {
    #[serde(borrow)]
    loc: HashMap<&'a str, Location<'a>>,
    events: Option<Vec<Event<'a>>>,
}

#[derive(Clone, serde::Deserialize)]
pub struct Location<'a> {
    pub name: &'a str,
}

#[derive(Eq, PartialEq, serde::Deserialize)]
pub struct Event<'a> {
    #[serde(deserialize_with = "de_from")]
    pub from: NaiveDate,
    pub loc: &'a str,
}

fn de_from<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
    use serde::Deserialize;
    use serde::de::Error;

    let date = toml::value::Datetime::deserialize(deserializer)?;
    Ok(date.to_string().parse().map_err(|_| D::Error::invalid_value(Unexpected::Other("non-date"), &"a date"))?)
}

impl<'a> PartialOrd for Event<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.from.partial_cmp(&other.from)
    }
}

impl<'a> Ord for Event<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.from.cmp(&other.from)
    }
}

pub fn now() -> Result<Option<(String, String, NaiveDate, Option<NaiveDate>)>, OuestError> {
    let now = chrono::Utc::now().naive_local().date();
    let file = std::fs::read_to_string("data/ouest.toml")?;
    let Data { events: mut events_vec, loc } = toml::from_str(&file)?;

    let events = events_vec.as_deref_mut().unwrap_or_default();
    events.sort();

    for (idx, before) in events.iter().enumerate() {
        let after = events.get(idx + 1);

        if now >= before.from && {
            match after {
                Some(after) => now < after.from,
                None => true,
            }
        } {
            let location = loc.get(before.loc).ok_or(OuestError::UndefinedLocation)?;
            return Ok(Some((
                location.name.to_string(),
                format!("/{}.png", before.loc),
                before.from.clone(),
                after.map(|e| e.from.clone()),
            )))
        }
    }

    Ok(None)
}
