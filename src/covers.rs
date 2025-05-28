use std::sync::Arc;
use tokio::sync::Mutex;

use crate::x1::X1;
#[derive(Clone, Debug)]
pub struct Blind {
    pub name: String,
    pub step_up_down: Option<StepUpDown>,
    pub up_down: Option<UpDown>,
    pub movement: Option<Movement>,
    pub position: Option<Position>,
}

impl Blind {
    pub async fn up(&self, x1: &X1) {
        let up_down_uid = self.up_down.clone().expect("Error getting UpDown").uid;

        let _res = x1.set_value(up_down_uid, 0).await;
    }
    pub async fn down(&self, x1: &X1) {
        let up_down_uid = self.up_down.clone().expect("Error getting UpDown").uid;

        let _res = x1.set_value(up_down_uid, 1).await;
    }
}

#[derive(Clone, Debug)]
pub struct Blinds {
    pub blinds: Arc<Mutex<Vec<Blind>>>,
}
impl Blinds {
    pub async fn list(&self) -> Vec<String> {
        //println!("Lights:");
        let mut list: Vec<String> = vec![];

        for (_index, blind) in self.blinds.lock().await.iter().enumerate() {
            list.push(blind.name.clone());
        }
        list
    }

    pub async fn get_all(&self) -> Vec<Blind> {
        self.blinds.lock().await.clone()
    }
}

#[derive(Clone, Debug)]
pub struct StepUpDown {
    pub uid: String,
    pub val: u16,
}

#[derive(Clone, Debug)]
pub struct UpDown {
    pub uid: String,
    pub val: u16,
}

#[derive(Clone, Debug)]
pub struct Position {
    pub uid: String,
    pub val: u16,
}

#[derive(Clone, Debug)]
pub struct Movement {
    pub uid: String,
    pub val: u16,
}
