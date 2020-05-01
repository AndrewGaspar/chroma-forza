use serde::{Serialize, Serializer};

#[repr(C)]
#[derive(Copy, Clone, Serialize)]
pub struct Vector<T> {
    x: T,
    y: T,
    z: T,
}

#[repr(C)]
#[derive(Copy, Clone, Serialize)]
pub struct Quad<T> {
    front_left: T,
    front_right: T,
    rear_left: T,
    rear_right: T,
}

#[repr(C)]
#[derive(Copy, Clone, Serialize)]
pub struct Sled {
    is_race_on: i32,
    timestamp_ms: u32,
    engine_max_rpm: f32,
    engine_idle_rpm: f32,
    current_engine_rpm: f32,
    acceleration: Vector<f32>,
    velocity: Vector<f32>,
    angular_velocity: Vector<f32>,
    yaw: f32,
    pitch: f32,
    roll: f32,
    normalized_suspension_travel: Quad<f32>,
    tire_split_ratio: Quad<f32>,
    wheel_rotation_speed: Quad<f32>,
    wheel_on_rumble_strip: Quad<i32>,
    wheel_in_puddle_depth: Quad<f32>,
    surface_rumble: Quad<f32>,
    tire_slip_angle: Quad<f32>,
    tire_combined_slip: Quad<f32>,
    suspension_travel_meters: Quad<f32>,
    car_ordinal: i32,
    car_class: i32,
    car_performance_index: i32,
    drivetrain_type: i32,
    num_cylinders: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Serialize)]
pub struct DashUnpacked {
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
#[derive(Copy, Clone)]
pub struct Dash {
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
#[derive(Copy, Clone, Serialize)]
pub struct Horizon4Datagram {
    sled: Sled,
    // Unsure, but first byte is always decimal 40 (0x28)
    unknown1: [u8; 4],
    // related to depub struction
    unknown2: f32,
    // related to depub struction
    unknown3: f32,
    dash: Dash,
    unknown4: i8,
}
