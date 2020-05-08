use serde::{Deserialize, Serialize, Serializer};

#[repr(C)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Quad<T> {
    pub front_left: T,
    pub front_right: T,
    pub rear_left: T,
    pub rear_right: T,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Sled {
    pub is_race_on: i32,
    pub timestamp_ms: u32,
    pub engine_max_rpm: f32,
    pub engine_idle_rpm: f32,
    pub current_engine_rpm: f32,
    pub acceleration: Vector<f32>,
    pub velocity: Vector<f32>,
    pub angular_velocity: Vector<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub normalized_suspension_travel: Quad<f32>,
    pub tire_split_ratio: Quad<f32>,
    pub wheel_rotation_speed: Quad<f32>,
    pub wheel_on_rumble_strip: Quad<i32>,
    pub wheel_in_puddle_depth: Quad<f32>,
    pub surface_rumble: Quad<f32>,
    pub tire_slip_angle: Quad<f32>,
    pub tire_combined_slip: Quad<f32>,
    pub suspension_travel_meters: Quad<f32>,
    pub car_ordinal: i32,
    pub car_class: i32,
    pub car_performance_index: i32,
    pub drivetrain_type: i32,
    pub num_cylinders: i32,
}

/// This struct just exists to derive `Serialize` so that it can be forwarded to from `Dash`.
#[repr(C)]
#[derive(Copy, Clone, Debug, Serialize)]
struct DashUnpacked {
    position: Vector<f32>,
    // m/s
    speed: f32,
    // watts
    power: f32,
    // Newton-meter
    torque: f32,
    tire_temp: Quad<f32>,
    boost: f32,
    fuel: f32,
    distance_traveled: f32,
    best_lap: f32,
    last_lap: f32,
    current_lap: f32,
    current_race_time: f32,
    lap_number: u16,
    race_position: u8,
    accel: u8,
    brake: u8,
    clutch: u8,
    hand_brake: u8,
    gear: u8,
    steer: i8,
    normalized_driving_line: i8,
    normalized_ai_brake_difference: i8,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Dash {
    pub position: Vector<f32>,
    // m/s
    pub speed: f32,
    // watts
    pub power: f32,
    // Newton-meter
    pub torque: f32,
    pub tire_temp: Quad<f32>,
    pub boost: f32,
    pub fuel: f32,
    pub distance_traveled: f32,
    pub best_lap: f32,
    pub last_lap: f32,
    pub current_lap: f32,
    pub current_race_time: f32,
    pub lap_number: u16,
    pub race_position: u8,
    pub accel: u8,
    pub brake: u8,
    pub clutch: u8,
    pub hand_brake: u8,
    pub gear: u8,
    pub steer: i8,
    pub normalized_driving_line: i8,
    pub normalized_ai_brake_difference: i8,
}

impl Serialize for Dash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        DashUnpacked {
            position: self.position,
            speed: self.speed,
            power: self.power,
            torque: self.torque,
            tire_temp: self.tire_temp,
            boost: self.boost,
            fuel: self.fuel,
            distance_traveled: self.distance_traveled,
            best_lap: self.best_lap,
            last_lap: self.last_lap,
            current_lap: self.current_lap,
            current_race_time: self.current_race_time,
            lap_number: self.lap_number,
            race_position: self.race_position,
            accel: self.accel,
            brake: self.brake,
            clutch: self.clutch,
            hand_brake: self.hand_brake,
            gear: self.gear,
            steer: self.steer,
            normalized_driving_line: self.normalized_driving_line,
            normalized_ai_brake_difference: self.normalized_ai_brake_difference,
        }
        .serialize(serializer)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Horizon4Datagram {
    pub sled: Sled,
    // Unsure, but first byte is always decimal 40 (0x28)
    pub unknown1: [u8; 4],
    // related to destruction
    pub unknown2: f32,
    // related to destruction
    pub unknown3: f32,
    pub dash: Dash,
    pub unknown4: i8,
}
