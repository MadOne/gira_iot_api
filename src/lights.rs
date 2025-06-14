use std::sync::Arc;
use tokio::sync::Mutex;

use crate::x1::X1;

#[derive(Clone, Debug)]
pub enum LightType {
    SWITCH,
    DIMM,
    TUNE,
    COLOR,
    UNKNOWN,
}
#[derive(Clone, Debug)]
pub struct Light {
    pub uid: String,
    pub name: String,
    pub lighttype: LightType,
    pub switch: Option<Switch>,
    pub dimmer: Option<Dimmer>,
    pub tuner: Option<Tuner>,
    pub color: Option<Color>,
    pub location: Option<u16>,
}

impl Light {
    pub async fn switch_on(&mut self, x1: &X1) {
        let switch_uid = self.switch.clone().expect("Error getting Switch").uid;
        let _res = x1.set_value(switch_uid, 1).await;
        self.switch.as_mut().unwrap().val = 1;
    }
    pub async fn switch_off(&mut self, x1: &X1) {
        let switch_uid = self.switch.clone().expect("Error getting Switch").uid;
        let _res = x1.set_value(switch_uid, 0).await;
        self.switch.as_mut().unwrap().val = 0;
    }

    pub async fn dimm(&mut self, x1: &X1, value: u16) {
        let dimm_uid = self.dimmer.clone().expect("Error getting Dimmer").uid;
        let _res = x1.set_value(dimm_uid, value).await;
        self.dimmer.as_mut().unwrap().val = value;
    }

    pub async fn tune(&mut self, x1: &X1, value: u16) {
        let tune_uid = self.tuner.clone().expect("Error getting Tuner").uid;
        let _res = x1.set_value(tune_uid, value).await;
        self.tuner.as_mut().unwrap().val = value;
    }
}
#[derive(Clone, Debug)]
pub struct Switch {
    pub uid: String,
    pub val: u16,
}
#[derive(Clone, Debug)]
pub struct Dimmer {
    pub uid: String,
    pub val: u16,
}
#[derive(Clone, Debug)]
pub struct Tuner {
    pub uid: String,
    pub val: u16,
}
#[derive(Clone, Debug)]
pub struct Color {
    pub uid: String,
    pub val: u16,
}
#[derive(Clone, Debug)]
pub struct Lights {
    pub light: Arc<Mutex<Vec<Light>>>,
}

impl Lights {
    pub async fn list(&self) -> Vec<String> {
        //println!("Lights:");
        let mut list: Vec<String> = vec![];

        for (_index, light) in self.light.lock().await.iter().enumerate() {
            let _kind = match light.lighttype {
                LightType::COLOR => ": (Colored light)",
                LightType::DIMM => ": (Dimmable Light)",
                LightType::SWITCH => ": (Switchable Light)",
                LightType::TUNE => ": (Tuneable Light)",
                LightType::UNKNOWN => ": (Unknown Light)",
            };
            //println!("{index}: {} {kind}", light.name);
            list.push(light.name.clone());
        }
        list
    }

    pub async fn get_all(&self) -> Vec<Light> {
        self.light.lock().await.clone()
    }
}

pub struct LightDetails {}
