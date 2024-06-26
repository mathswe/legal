// Copyright (c) 2024 Tobias Briones. All rights reserved.
// This file is part of https://github.com/mathswe/legal

use std::str::FromStr;
use serde::{Deserialize, Serialize};
use worker::Request;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Geolocation {
    #[serde(with = "chrono_tz_serde")]
    time_zone: chrono_tz::Tz,
    country: Option<String>,
    city: Option<String>,
    region: Option<String>,
    region_code: Option<String>,
}

impl Geolocation {
    #[allow(dead_code)]
    pub fn empty_with(time_zone: chrono_tz::Tz) -> Self {
        Geolocation {
            time_zone,
            country: None,
            city: None,
            region: None,
            region_code: None,
        }
    }

    pub fn from_req(req: &Request) -> Self {
        let cf = req.cf().unwrap();

        Geolocation {
            time_zone: chrono_tz::Tz::from_str(&cf.timezone_name()).unwrap(),
            country: cf.country(),
            city: cf.city(),
            region: cf.region(),
            region_code: cf.region_code(),
        }
    }
}

mod chrono_tz_serde {
    use std::str::FromStr;
    use chrono_tz::Tz;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(
        tz: &Tz,
        serializer: S,
    ) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(tz.name())
    }

    pub fn deserialize<'de, D>(
        deserializer: D
    ) -> Result<Tz, D::Error> where D: Deserializer<'de> {
        let tz_str = String::deserialize(deserializer)?;

        Tz::from_str(&tz_str).map_err(serde::de::Error::custom)
    }
}
