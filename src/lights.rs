use std::sync::Arc;
use tokio::sync::Mutex;

use crate::x1::{self, X1};

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
}

impl Light {
    pub async fn switch_on(&self, x1: &X1) {
        let switch_uid = self.switch.clone().expect("Error getting Switch").uid;

        x1.set_value(switch_uid, 1).await;
    }
    pub async fn switch_off(&self, x1: &X1) {
        let switch_uid = self.switch.clone().expect("Error getting Switch").uid;
        let res = x1.set_value(switch_uid, 0).await;
        println!("{res:?}")
    }

    pub async fn dimm(&self, x1: &X1, value: u16) {
        let dimm_uid = self.dimmer.clone().expect("Error getting Dimmer").uid;
        x1.set_value(dimm_uid, value).await;
    }

    pub async fn tune(&self, x1: &X1, value: u16) {
        let tune_uid = self.tuner.clone().expect("Error getting Tuner").uid;
        x1.set_value(tune_uid, value).await;
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

        for (index, light) in self.light.lock().await.iter().enumerate() {
            let kind = match light.lighttype {
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
}
