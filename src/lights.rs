use std::sync::{Arc, Mutex};

use crate::x1;

pub struct SwitchedLight<'a> {
    pub x1: &'a x1::X1<'a>,
    pub name: String,
    pub switch: Option<Switch<'a>>,
}

pub struct DimmedLight<'a> {
    pub x1: &'a x1::X1<'a>,
    pub name: String,
    pub switch: Option<Switch<'a>>,
    pub dimmer: Option<Dimm<'a>>,
}
#[derive(Clone)]
pub struct TunableLight<'a> {
    pub x1: &'a x1::X1<'a>,
    pub name: String,
    pub switch: Option<Switch<'a>>,
    pub dimmer: Option<Dimm<'a>>,
    pub tuner: Option<ColorTemp<'a>>,
}

pub struct SwitchableLights<'a> {
    switchable: Arc<Mutex<Vec<SwitchedLight<'a>>>>,
}
impl<'a> SwitchableLights<'a> {
    pub fn new() -> Self {
        let vector: Vec<SwitchedLight> = vec![];
        let mutex = Mutex::new(vector);
        let arc = Arc::new(mutex);
        SwitchableLights { switchable: arc }
    }
    pub fn add(&'a self, light: SwitchedLight<'a>) {
        self.switchable.lock().unwrap().push(light);
    }
    pub fn list(&self) {
        println!("Switchable Lights:");
        for (index, switchable) in self.switchable.lock().unwrap().iter().enumerate() {
            println!("{index}: {}", switchable.name);
        }
    }
    pub async fn switch_on(&'a self, id: u8) {
        self.switchable
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .switch
            .clone()
            .expect("Function On/Off not setup. Missing Datapoints")
            .on()
            .await;
    }
    pub async fn switch_off(&'a self, id: u8) {
        self.switchable
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .switch
            .clone()
            .expect("Function On/Off not setup. Missing Datapoints")
            .off()
            .await;
    }
}

pub struct DimmableLights<'a> {
    pub dimmable: Arc<Mutex<Vec<DimmedLight<'a>>>>,
}
impl<'a> DimmableLights<'a> {
    pub fn new() -> Self {
        let vector: Vec<DimmedLight> = vec![];
        let mutex = Mutex::new(vector);
        let arc = Arc::new(mutex);
        DimmableLights { dimmable: arc }
    }
    pub fn add(&'a self, light: DimmedLight<'a>) {
        self.dimmable.lock().unwrap().push(light);
    }
    pub fn list(&self) {
        println!("Dimmable Lights:");
        for (index, dimmable) in self.dimmable.lock().unwrap().iter().enumerate() {
            println!("{index}: {}", dimmable.name);
        }
    }
    pub async fn switch_on(&'a self, id: u8) {
        self.dimmable
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .switch
            .clone()
            .expect("Function On/Off not setup. Missing Datapoints")
            .on()
            .await;
    }
    pub async fn switch_off(&'a self, id: u8) {
        self.dimmable
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .switch
            .clone()
            .expect("Function On/Off not setup. Missing Datapoints")
            .off()
            .await;
    }
    pub async fn dimm(&'a self, id: u8, val: u16) {
        self.dimmable
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .dimmer
            .clone()
            .expect("Function Dimming not setup. Missing Datapoints")
            .set_val(val)
            .await;
    }
}
pub struct TunableLights<'a> {
    pub tuneable: Arc<Mutex<Vec<TunableLight<'a>>>>,
}
impl<'a> TunableLights<'a> {
    pub fn new() -> Self {
        let vector: Vec<TunableLight> = vec![];
        let mutex = Mutex::new(vector);
        let arc = Arc::new(mutex);
        TunableLights { tuneable: arc }
    }
    pub fn add(&'a self, light: TunableLight<'a>) {
        self.tuneable.lock().unwrap().push(light);
    }
    pub fn list(&self) {
        println!("Tunable Lights:");
        for (index, tuneable) in self.tuneable.lock().unwrap().iter().enumerate() {
            println!("{index}: {}", tuneable.name);
        }
    }
    pub async fn switch_on(&'a self, id: u8) {
        self.tuneable
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .switch
            .clone()
            .expect("Function On/Off not setup. Missing Datapoints")
            .on()
            .await;
    }
    pub async fn switch_off(&'a self, id: u8) {
        self.tuneable
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .switch
            .clone()
            .expect("Function On/Off not setup. Missing Datapoints")
            .off()
            .await;
    }
    pub async fn dimm(&'a self, id: u8, val: u16) {
        self.tuneable
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .dimmer
            .clone()
            .expect("Function Dimming not setup. Missing Datapoints")
            .set_val(val)
            .await;
    }
    pub async fn tune(&'a self, id: u8, color: u16) {
        self.tuneable
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .tuner
            .clone()
            .expect("Function ColorTemp not setup. Missing Datapoints")
            .set_val(color)
            .await;
    }
}
pub struct Lights<'a> {
    pub switchable: SwitchableLights<'a>,
    pub dimmable: DimmableLights<'a>,
    pub tunable: TunableLights<'a>,
}
impl Lights<'_> {
    pub fn new() -> Self {
        Lights {
            switchable: SwitchableLights::new(),
            dimmable: DimmableLights::new(),
            tunable: TunableLights::new(),
        }
    }
    pub fn list(&self) {
        self.switchable.list();
        self.dimmable.list();
        self.tunable.list();
    }
}

pub struct ColoredLight {}

#[derive(Clone)]
pub struct Switch<'a> {
    pub x1: &'a x1::X1<'a>,
    pub uid: String,
    pub val: u32,
}
impl Switch<'_> {
    pub async fn on(&self) {
        self.x1
            .set_value(self.uid.clone(), 1)
            .await
            .expect("error switching on light");
    }
    pub async fn off(&self) {
        self.x1
            .set_value(self.uid.clone(), 0)
            .await
            .expect("error switching on light");
    }
    pub async fn refresh_val(&mut self) {
        self.x1
            .get_value(self.uid.clone())
            .await
            .expect("Error refreshing value");
    }
}

#[derive(Clone)]
pub struct Dimm<'a> {
    pub x1: &'a x1::X1<'a>,
    pub uid: String,
    pub val: u32,
}
impl Dimm<'_> {
    pub async fn set_val(&self, val: u16) {
        self.x1
            .set_value(self.uid.clone(), val)
            .await
            .expect("error switching on light");
    }

    pub async fn refresh_val(&mut self) {
        self.x1
            .get_value(self.uid.clone())
            .await
            .expect("Error refreshing value");
    }
}

#[derive(Clone)]
pub struct ColorTemp<'a> {
    pub x1: &'a x1::X1<'a>,
    pub uid: String,
    pub val: u32,
}

impl ColorTemp<'_> {
    pub async fn set_val(&self, val: u16) {
        self.x1
            .set_value(self.uid.clone(), val)
            .await
            .expect("error switching on light");
    }

    pub async fn refresh_val(&mut self) {
        self.x1
            .get_value(self.uid.clone())
            .await
            .expect("Error refreshing value");
    }
}

/*
pub struct Color {
    uid: String,
    val: u32,
}
*/
