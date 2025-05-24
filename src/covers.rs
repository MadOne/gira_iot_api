use std::sync::{Arc, Mutex};

use crate::x1;

pub struct Blind<'a> {
    pub x1: &'a x1::X1<'a>,
    pub name: String,
    pub step_up_down: StepUpDown<'a>,
    pub up_down: UpDown<'a>,
    pub position: Position<'a>,
}

pub struct Blinds<'a> {
    pub blinds: Arc<Mutex<Vec<Blind<'a>>>>,
}
impl<'a> Blinds<'a> {
    pub fn new() -> Self {
        let vector: Vec<Blind> = vec![];
        let mutex = Mutex::new(vector);
        let arc = Arc::new(mutex);
        Blinds { blinds: arc }
    }
    pub fn add(&'a self, light: Blind<'a>) {
        self.blinds.lock().unwrap().push(light);
    }
    pub fn list(&self) {
        println!("Blinds:");
        for (index, tuneable) in self.blinds.lock().unwrap().iter().enumerate() {
            println!("{index}: {}", tuneable.name);
        }
    }
    pub async fn step_up(&'a self, id: u8) {
        self.blinds
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .step_up_down
            .step_up()
            .await;
    }
    pub async fn step_down(&'a self, id: u8) {
        self.blinds
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .step_up_down
            .step_down()
            .await;
    }
    pub async fn up(&'a self, id: u8) {
        self.blinds
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .up_down
            .up()
            .await;
    }
    pub async fn down(&'a self, id: u8) {
        self.blinds
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .up_down
            .down()
            .await;
    }
    pub async fn position(&'a self, id: u8, position: u16) {
        self.blinds
            .lock()
            .unwrap()
            .get(id as usize)
            .unwrap()
            .position
            .set_val(position)
            .await;
    }
}

#[derive(Clone)]
pub struct StepUpDown<'a> {
    pub x1: &'a x1::X1<'a>,
    pub uid: String,
    pub val: u32,
}
impl StepUpDown<'_> {
    pub async fn step_up(&self) {
        self.x1
            .set_value(self.uid.clone(), 0)
            .await
            .expect("error switching on light");
    }
    pub async fn step_down(&self) {
        self.x1
            .set_value(self.uid.clone(), 1)
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
pub struct UpDown<'a> {
    pub x1: &'a x1::X1<'a>,
    pub uid: String,
    pub val: u32,
}
impl UpDown<'_> {
    pub async fn up(&self) {
        self.x1
            .set_value(self.uid.clone(), 0)
            .await
            .expect("error opening blind");
    }
    pub async fn down(&self) {
        self.x1
            .set_value(self.uid.clone(), 1)
            .await
            .expect("error closing blind");
    }

    pub async fn refresh_val(&mut self) {
        self.x1
            .get_value(self.uid.clone())
            .await
            .expect("Error refreshing value");
    }
}

#[derive(Clone)]
pub struct Position<'a> {
    pub x1: &'a x1::X1<'a>,
    pub uid: String,
    pub val: u32,
}

impl Position<'_> {
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
