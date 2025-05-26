use std::sync::Arc;
use tokio::sync::Mutex;

use crate::x1;

#[derive(Clone, Debug)]
pub struct SwitchedLight<'a> {
    pub x1: &'a x1::X1<'a>,
    pub name: String,
    pub switch: Option<Switch<'a>>,
}
#[derive(Clone, Debug)]
pub struct DimmedLight<'a> {
    pub x1: &'a x1::X1<'a>,
    pub name: String,
    pub switch: Option<Switch<'a>>,
    pub dimmer: Option<Dimm<'a>>,
}
#[derive(Clone, Debug)]
pub struct TunableLight<'a> {
    pub x1: &'a x1::X1<'a>,
    pub name: String,
    pub switch: Option<Switch<'a>>,
    pub dimmer: Option<Dimm<'a>>,
    pub tuner: Option<ColorTemp<'a>>,
}

#[derive(Clone, Debug)]
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
        self.switchable.try_lock().unwrap().push(light);
    }
    pub fn list(&self) {
        println!("Switchable Lights:");
        for (index, switchable) in self.switchable.try_lock().unwrap().iter().enumerate() {
            println!("{index}: {}", switchable.name);
        }
    }
    pub async fn switch_on(&'a self, id: u8) {
        self.switchable
            .try_lock()
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
            .try_lock()
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

#[derive(Clone, Debug)]
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
        self.dimmable.try_lock().unwrap().push(light);
    }
    pub fn list(&self) {
        println!("Dimmable Lights:");
        for (index, dimmable) in self.dimmable.try_lock().unwrap().iter().enumerate() {
            println!("{index}: {}", dimmable.name);
        }
    }
    pub async fn switch_on(&'a self, id: u8) {
        self.dimmable
            .try_lock()
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
            .try_lock()
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
            .try_lock()
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
#[derive(Clone, Debug)]
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
        self.tuneable.try_lock().unwrap().push(light);
    }
    pub fn list(&self) -> Vec<String> {
        println!("Tunable Lights:");
        let mut list: Vec<String> = vec![];
        for (index, tuneable) in self.tuneable.try_lock().unwrap().iter().enumerate() {
            println!("{index}: {}", tuneable.name);
            list.push(tuneable.name.clone());
        }
        list
    }
    pub async fn switch_on(&'a self, id: u8) {
        self.tuneable
            .try_lock()
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
            .try_lock()
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
            .try_lock()
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
            .try_lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .tuner
            .clone()
            .expect("Function ColorTemp not setup. Missing Datapoints")
            .set_val(color)
            .await;
    }

    pub fn get_on_off(&'a self, id: u8) -> u16 {
        self.tuneable
            .try_lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .switch
            .clone()
            .expect("Function ColorTemp not setup. Missing Datapoints")
            .get_val()
    }
    pub fn get_dimm(&'a self, id: u8) -> u16 {
        self.tuneable
            .try_lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .dimmer
            .clone()
            .expect("Function ColorTemp not setup. Missing Datapoints")
            .get_val()
    }

    pub fn get_tune(&'a self, id: u8) -> u16 {
        self.tuneable
            .try_lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .tuner
            .clone()
            .expect("Function ColorTemp not setup. Missing Datapoints")
            .get_val()
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Switch<'a> {
    pub x1: &'a x1::X1<'a>,
    pub uid: String,
    pub val: u16,
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
    pub fn get_val(&self) -> u16 {
        self.val
    }
}

#[derive(Clone, Debug)]
pub struct Dimm<'a> {
    pub x1: &'a x1::X1<'a>,
    pub uid: String,
    pub val: u16,
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

    pub fn get_val(&self) -> u16 {
        self.val
    }
}

#[derive(Clone, Debug)]
pub struct ColorTemp<'a> {
    pub x1: &'a x1::X1<'a>,
    pub uid: String,
    pub val: u16,
}

impl ColorTemp<'_> {
    pub async fn set_val(&self, val: u16) {
        self.x1
            .set_value(self.uid.clone(), val)
            .await
            .expect("error switching on light");
    }

    pub async fn refresh_val(&mut self) {
        self.val = self
            .x1
            .get_value(self.uid.clone())
            .await
            .expect("Error refreshing value");
    }

    pub fn get_val(&self) -> u16 {
        self.val
    }
}

/*
pub struct Color {
    uid: String,
    val: u32,
}
*/
