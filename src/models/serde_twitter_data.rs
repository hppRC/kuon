pub(crate) mod twitter_date {
    use chrono::{DateTime, Utc};
    use serde::de::{Unexpected, Visitor};
    use serde::{Deserializer, Serializer};

    struct TwitterDateTimeVisitor;

    impl<'de> Visitor<'de> for TwitterDateTimeVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            formatter.write_str("a string in the format by \"%a %b %d %H:%M:%S %z %Y\"")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            chrono::DateTime::parse_from_str(v, "%a %b %d %H:%M:%S %z %Y")
                .map(|x| x.with_timezone(&Utc))
                .map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(v), &self))
        }
    }

    pub fn serialize<S>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.format("%a %b %d %H:%M:%S %z %Y").to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TwitterDateTimeVisitor)
    }
}

pub(crate) mod optional_twitter_date {
    use chrono::{DateTime, Utc};
    use serde::de::{Unexpected, Visitor};
    use serde::{Deserializer, Serializer};

    struct OptionalTwitterDateTimeVisitor;

    impl<'de> Visitor<'de> for OptionalTwitterDateTimeVisitor {
        type Value = Option<DateTime<Utc>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            formatter.write_str("a string in the format by \"%a %b %d %H:%M:%S %z %Y\"")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if v.is_empty() {
                Ok(None)
            } else {
                chrono::DateTime::parse_from_str(v, "%a %b %d %H:%M:%S %z %Y")
                    .map(|x| Some(x.with_timezone(&Utc)))
                    .map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(v), &self))
            }
        }
    }

    pub fn serialize<S>(value: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(date_time) => {
                serializer.serialize_str(&date_time.format("%a %b %d %H:%M:%S %z %Y").to_string())
            }
            None => serializer.serialize_str(""),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(OptionalTwitterDateTimeVisitor)
    }
}
