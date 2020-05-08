use std::{cell::Cell, collections::HashMap, iter::FromIterator};

use forza::Horizon4Datagram;

use crate::config;

pub fn query_rate_property(config: &config::Input) -> RateProperty {
    let query = &RATE_PROPERTIES[&config.property[..]];

    let max_value = if let Some(max) = config.max_value {
        Some(Cell::new(max))
    } else if config.auto_raise {
        Some(Cell::new(0.0))
    } else {
        None
    };

    RateProperty {
        query,
        max_value,
        auto_raise: config.auto_raise,
    }
}

lazy_static::lazy_static! {
    static ref RATE_PROPERTIES: HashMap<&'static str, RatePropertyQuery>
        = rate_properties();
}

fn rate_properties() -> HashMap<&'static str, RatePropertyQuery> {
    HashMap::from_iter(vec![
        (
            "speed",
            RatePropertyQuery {
                current: |datagram| datagram.dash.speed,
                max: None,
            },
        ),
        (
            "rpm",
            RatePropertyQuery {
                current: |datagram| datagram.sled.current_engine_rpm,
                max: Some(|datagram| datagram.sled.engine_max_rpm),
            },
        ),
        (
            "rpm-baseline",
            RatePropertyQuery {
                current: |datagram| {
                    datagram.sled.current_engine_rpm - datagram.sled.engine_idle_rpm
                },
                max: Some(|datagram| datagram.sled.engine_max_rpm - datagram.sled.engine_idle_rpm),
            },
        ),
        (
            "driveline",
            RatePropertyQuery {
                current: |datagram| {
                    let line = datagram.dash.normalized_driving_line as i16;
                    let line = line + 127;
                    line as f32
                },
                max: Some(|_| 255.0),
            },
        ),
    ])
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

// trait RateProperty {
//     fn read_value(&self, datagram: &Horizon4Datagram) -> f32;
// }

// struct ScalarRateProperty<F> {
//     f: fn(&Horizon4Datagram) -> f32,
//     max_value: Option<std::cell::Cell<f32>>,
// }

// impl ScalarRateProperty<F: Fn(&Horizon4Datagram) -> f32> {
//     fn new(f: fn(&Horizon4Datagram) -> f32) -> Self {
//         Self { f, max_value: None }
//     }
// }
