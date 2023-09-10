// 定义一个 TrafficLightTrait trait，其中包含一个 get_duration 方法
pub trait TrafficLightTrait {
    fn get_duration(&self) -> u32;
}

// 为红灯（RedLight）实现 TrafficLightTrait
pub struct RedLight;

impl TrafficLightTrait for RedLight {
    fn get_duration(&self) -> u32 {
        60 // 红灯持续60秒
    }
}

// 为黄灯（YellowLight）实现 TrafficLightTrait
pub struct YellowLight;

impl TrafficLightTrait for YellowLight {
    fn get_duration(&self) -> u32 {
        10 // 黄灯持续10秒
    }
}

// 为绿灯（GreenLight）实现 TrafficLightTrait
pub struct GreenLight;

impl TrafficLightTrait for GreenLight {
    fn get_duration(&self) -> u32 {
        30 // 绿灯持续30秒
    }
}

// 主函数，用于测试
pub fn start() {
    let red_light = RedLight;
    let yellow_light = YellowLight;
    let green_light = GreenLight;

    println!("Red light duration: {} seconds", red_light.get_duration());
    println!("Yellow light duration: {} seconds", yellow_light.get_duration());
    println!("Green light duration: {} seconds", green_light.get_duration());
}
