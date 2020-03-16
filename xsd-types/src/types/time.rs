use crate::utils;
use crate::types::utils::parse_timezone;
use chrono::{NaiveTime, format::strftime::StrftimeItems, FixedOffset};
use std::fmt;
use std::io::{Read, Write};
use std::str::FromStr;
use yaserde::{YaDeserialize, YaSerialize};

#[derive(PartialEq, Debug)]
pub struct Time {
    pub value: NaiveTime,
    pub timezone: Option<FixedOffset>,
}

impl Time {
    pub fn from_chrono_naive_time(time: NaiveTime) -> Self {
        Time { value: time, timezone: None }
    }

    pub fn to_chrono_naive_time(&self) -> NaiveTime {
        self.value
    }
}

impl Default for Time {
  fn default() -> Time {
    Self{ value: NaiveTime::from_hms(0, 0, 0), timezone: None }
  }
}

impl FromStr for Time {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_naive_time(s: &str) -> Result<NaiveTime, String> {
            NaiveTime::parse_from_str(s, "%H:%M:%S").map_err(|e| e.to_string())
        }

        if s.ends_with("Z") {
            return Ok(Time {
                value: parse_naive_time(&s[..s.len()-1])?,
                timezone: Some(FixedOffset::east(0))
            });
        }

        if s.contains("+") {
            if s.matches("+").count() > 1 {
                return Err("bad date format".to_string());
            }

            let idx: usize = s.match_indices("+").collect::<Vec<_>>()[0].0;
            let time_token = &s[..idx];
            let tz_token = &s[idx..];
            return Ok(Time {
                value: parse_naive_time(time_token)?,
                timezone: Some(parse_timezone(tz_token)?)
            });
        }

        if s.contains("-") {
            if s.matches("-").count() > 1 {
                return Err("bad date format".to_string());
            }

            let idx: usize = s.match_indices("-").collect::<Vec<_>>()[0].0;
            let time_token = &s[..idx];
            let tz_token = &s[idx..];
            return Ok(Time {
                value: parse_naive_time(time_token)?,
                timezone: Some(parse_timezone(tz_token)?)
            });
        }

        Ok(Time {
            value: parse_naive_time(s)?,
            timezone: None
        })
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt = StrftimeItems::new("%H:%M:%S");
        match self.timezone {
            Some(tz) =>  write!(f, "{}{}", self.value.format_with_items(fmt.clone()), tz),
            None =>  write!(f, "{}", self.value.format_with_items(fmt.clone()))
        }
    }
}

impl YaDeserialize for Time {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        utils::yaserde::deserialize(reader, |s| Time::from_str(s).map_err(|e| e.to_string()))
    }
}

impl YaSerialize for Time {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        utils::yaserde::serialize(self, "Time", writer, |s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;

    #[test]
    fn time_parse_test() {
        // No timezone.
        assert_eq!(Time::from_str("04:40:00"), Ok(Time{ value: NaiveTime::from_hms(4, 40, 0), timezone: None }));

        // Timezone "Z".
        assert_eq!(Time::from_str("04:40:00Z"), Ok(Time{ value: NaiveTime::from_hms(4, 40, 0), timezone: Some(FixedOffset::east(0)) }));

        // Positive offset.
        assert_eq!(Time::from_str("04:40:00+06:30"), Ok(Time{ value: NaiveTime::from_hms(4, 40, 0), timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60)) }));

        // Negative offset.
        assert_eq!(Time::from_str("04:40:00-06:30"), Ok(Time{ value: NaiveTime::from_hms(4, 40, 0), timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)) }));
    }

    #[test]
    fn time_display_test() {
        // No timezone.
        assert_eq!(Time{ value: NaiveTime::from_hms(4, 40, 0), timezone: None }.to_string(), "04:40:00");

        // Timezone "Z".
        assert_eq!(Time{ value: NaiveTime::from_hms(4, 40, 0), timezone: Some(FixedOffset::east(0)) }.to_string(), "04:40:00+00:00");

        // Positive offset.
        assert_eq!(Time{ value: NaiveTime::from_hms(4, 40, 0), timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60)) }.to_string(), "04:40:00+06:30");

        // Negative offset.
        assert_eq!(Time{ value: NaiveTime::from_hms(4, 40, 0), timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)) }.to_string(), "04:40:00-06:30");
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespace = "t: test")]
    pub struct Message {
        #[yaserde(prefix = "t", rename = "CreatedAt")]
        pub created_at: Time,

        #[yaserde(prefix = "t", rename = "Text")]
        pub text: String,
    }

    #[test]
    fn datetime_serialize_test() {
        let expected = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>04:40:00+06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m = Message {
            created_at: Time{ value: NaiveTime::from_hms(4, 40, 0), timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60)) },
            text: "Hello world".to_string(),
        };
        let actual = yaserde::ser::to_string(&m).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn integer_deserialize_test() {
        let s = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>04:40:00-06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m: Message = yaserde::de::from_str(&s).unwrap();
        assert_eq!(m.created_at.value, NaiveTime::from_hms(4, 40, 0));
        assert_eq!(m.created_at.timezone, Some(FixedOffset::west(6 * 3600 + 30 * 60)));
        assert_eq!(m.text, "Hello world".to_string());
    }
}
