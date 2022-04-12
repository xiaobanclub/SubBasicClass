enum TrafficLight {
    GREEN,
    YELLOW,
    RED,
}

trait DurationTime {
    fn duration_time(&self) -> u32;
}

impl DurationTime for TrafficLight {
    fn duration_time(&self) -> u32 {
        match self {
            TrafficLight::GREEN => 10,
            TrafficLight::YELLOW => 20,
            TrafficLight::RED => 30,
        }
    }
}

fn main() {
    println!("{}", TrafficLight::GREEN.duration_time());
    println!("{}", TrafficLight::YELLOW.duration_time());
    println!("{}", TrafficLight::RED.duration_time());
}
