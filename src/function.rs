use crate::covers::Blind;
use crate::lights::Light;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub enum X1Function {
    LIGHT(Light),
    BLIND(Blind),
}

#[derive(Clone, Debug)]
pub struct X1Functions {
    pub functions: Arc<Mutex<HashMap<String, X1Function>>>,
}
