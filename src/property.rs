use std::{cell::Cell, collections::HashMap, iter::FromIterator};

use forza::Horizon4Datagram;

use crate::config;

pub enum Property {
    Rate(RateProperty),
    Score(ScoreProperty),
}

pub fn query_property(config: &config::Input) -> Property {
    let query = &PROPERTIES[&config.property[..]];

    let max_value = if let Some(max) = config.max_value {
        Some(Cell::new(max))
    } else if config.auto_raise {
        Some(Cell::new(0.0))
    } else {
        None
    };

    match query {
        PropertyQuery::Rate(query) => Property::Rate(RateProperty {
            query,
            max_value,
            auto_raise: config.auto_raise,
        }),
        PropertyQuery::Score(query) => Property::Score(ScoreProperty { query }),
    }
}

lazy_static::lazy_static! {
    static ref PROPERTIES: HashMap<&'static str, PropertyQuery>
        = properties();
}

fn properties() -> HashMap<&'static str, PropertyQuery> {
    HashMap::from_iter(vec![
        (
            "speed",
            PropertyQuery::Rate(RatePropertyQuery {
                current: |datagram| datagram.dash.speed,
                max: None,
            }),
        ),
        (
            "rpm",
            PropertyQuery::Rate(RatePropertyQuery {
                current: |datagram| datagram.sled.current_engine_rpm,
                max: Some(|datagram| datagram.sled.engine_max_rpm),
            }),
        ),
        (
            "rpm-baseline",
            PropertyQuery::Rate(RatePropertyQuery {
                current: |datagram| {
                    datagram.sled.current_engine_rpm - datagram.sled.engine_idle_rpm
                },
                max: Some(|datagram| datagram.sled.engine_max_rpm - datagram.sled.engine_idle_rpm),
            }),
        ),
        (
            "driveline",
            PropertyQuery::Rate(RatePropertyQuery {
                current: |datagram| {
                    let line = datagram.dash.normalized_driving_line as i16;
                    let line = line + 127;
                    line as f32
                },
                max: Some(|_| 255.0),
            }),
        ),
        (
            "position",
            PropertyQuery::Score(ScorePropertyQuery {
                current: |datagram| (datagram.dash.race_position as i32) - 1,
            }),
        ),
    ])
}

enum PropertyQuery {
    Rate(RatePropertyQuery),
    Score(ScorePropertyQuery),
}

struct RatePropertyQuery {
    current: fn(&Horizon4Datagram) -> f32,
    max: Option<fn(&Horizon4Datagram) -> f32>,
}

pub struct RateProperty {
    query: &'static RatePropertyQuery,
    max_value: Option<Cell<f32>>,
    auto_raise: bool,
}

impl RateProperty {
    pub fn query(&self, datagram: &Horizon4Datagram) -> f32 {
        let current = (self.query.current)(datagram);
        let max = match self.query.max {
            Some(query) => query(datagram),
            None => {
                let max_value = self.max_value.as_ref().unwrap();
                if self.auto_raise {
                    if current > max_value.get() {
                        max_value.set(current);
                    }
                }

                max_value.get()
            }
        };

        // bound the ratio between 0.0 and 1.0
        (current / max).min(1.0).max(0.0)
    }
}

struct ScorePropertyQuery {
    current: fn(&Horizon4Datagram) -> i32,
}

pub struct ScoreProperty {
    query: &'static ScorePropertyQuery,
}

impl ScoreProperty {
    pub fn query(&self, datagram: &Horizon4Datagram) -> i32 {
        (self.query.current)(datagram)
    }
}
