use crate::covers::*;
use crate::lights::*;
use crate::locations::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct X1 {
    addr: String,
    user: String,
    password: String,
    client: reqwest::Client,
    token: Arc<Mutex<Option<String>>>,
    ui: Arc<Mutex<Option<UiResponse>>>,
    pub lights: Lights,
    pub blinds: Blinds,
    pub locations: Locations,
    last_location: Arc<Mutex<u16>>,
    pub connected: Arc<Mutex<bool>>,
}

impl X1 {
    pub fn new(addr: &str, user: &str, password: &str) -> Self {
        let client: reqwest::Client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Error creating client");
        let mymutex: Mutex<Option<String>> = Mutex::new(None);
        let myarc = Arc::new(mymutex);
        let mymutex2: Mutex<Option<UiResponse>> = Mutex::new(None);
        let myarc2 = Arc::new(mymutex2);

        X1 {
            addr: addr.to_string(),
            user: user.to_string(),
            password: password.to_string(),
            client,
            token: myarc,
            ui: myarc2,
            lights: Lights {
                light: Arc::new(Mutex::new(vec![])),
            },
            blinds: Blinds {
                blinds: Arc::new(Mutex::new(vec![])),
            },
            locations: Locations {
                locations: Arc::new(Mutex::new(HashMap::new())),
                set: Arc::new(Mutex::new(false)),
            },
            last_location: Arc::new(Mutex::new(0)),
            connected: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn connect_x1(&self) {
        if self.token.lock().await.is_some() {
            println!("Already connected. Skipping");
            return;
        }

        let body = "{\"client\":\"de.madone.x1client\"}";
        let addr = self.addr.clone();
        let token_json_str = self
            .client
            .post(format!("https://{addr}/api/clients"))
            .basic_auth(self.user.clone(), Some(self.password.clone()))
            .body(body)
            .send()
            .await
            .expect("Error sending post for auth")
            .text()
            .await
            .expect("Error getting text int auth");
        let token_hash: HashMap<String, String> =
            serde_json::from_str(token_json_str.as_str()).expect("invalid json");
        let token = token_hash
            .get("token")
            .expect("getting json from response failed");
        let myarc = self.token.clone();
        let mut mymutex = myarc.try_lock().expect("could not lock the mutex");
        *mymutex = Some(token.to_owned());
    }

    pub fn get_token(&self) -> Option<String> {
        let myarc = self.token.clone();
        let mymutex = myarc.try_lock().expect("could not lock the mutex");
        mymutex.clone()
    }

    pub async fn get_ui(&self) -> String {
        if self.ui.clone().lock().await.clone().is_some() {
            println!("Already polled ui. Skipping");
            return "".to_string();
        }

        let token = self
            .get_token()
            .expect("failed to get token. Not connected?");
        let addr = self.addr.clone();
        let resp = self.client
        .get(
            format!("https://{addr}/api/v2/uiconfig?expand=[dataPointFlasgs,parameters,locations,trades&token={token}"),
        )
        .send()
        .await
        .expect("failed to get response of UI")
        .text()
        .await
        .expect("failed to get text of UI response");
        let myresp: UiResponse = serde_json::from_str(&resp).unwrap();
        let myarc = self.ui.clone();
        let mut mymutex = myarc.try_lock().expect("could not lock the mutex");
        *mymutex = Some(myresp);
        resp
    }

    pub async fn get_value(&self, uid: String) -> Result<u16, reqwest::Error> {
        let token = self
            .get_token()
            .expect("Error geting token. Not logged in?");
        let addr = self.addr.clone();
        if true {
            let resp = self
                .client
                .get(format!("https://{addr}/api/v2/values/{uid}?token={token}"))
                .send()
                .await?
                .text()
                .await?;

            let myresp: Value = serde_json::from_str(&resp).unwrap();
            //println!("{:?}", myresp);
            if let Some(val) = myresp.values {
                let a = val[0].get("value").unwrap();
                let a: Vec<&str> = a.split('.').collect();
                let a = a[0].to_string();
                let b: u16 = a.parse().unwrap();
                return Ok(b);
            }
        }
        Ok(0)
    }
    pub async fn get_fn_values(&self, uid: String) -> Result<HashMap<String, u16>, reqwest::Error> {
        let token = self
            .get_token()
            .expect("Error geting token. Not logged in?");
        let addr = self.addr.clone();
        let mut values: HashMap<String, u16> = HashMap::new();
        if true {
            let resp = self
                .client
                .get(format!("https://{addr}/api/v2/values/{uid}?token={token}"))
                .send()
                .await?
                .text()
                .await?;

            let myresp: Value = serde_json::from_str(&resp).unwrap();

            for val in myresp.values.unwrap_or(vec![]) {
                //let key = val.get("uid").unwrap();
                let value: u16 = val.get("value").unwrap().parse().unwrap_or(0);
                values.insert(val.get("uid").unwrap().to_owned(), value);
            }
            //println!("{:?}", values);
        }
        Ok(values)
    }

    pub async fn set_value(&self, uid: String, value: u16) -> Result<String, reqwest::Error> {
        let token = self.get_token().expect("Error getting token");
        let addr = self.addr.clone();
        let body = format!(
            "{{
                \"values\": [
                    {{
                        \"uid\": \"{uid}\",
                        \"value\": {value}
                    }}
                ]
            }}"
        );
        let resp = self
            .client
            .put(format!("https://{addr}/api/v2/values?token={token}"))
            //.basic_auth("MadOne", Some("5315herb"))
            .body(body)
            .send()
            .await?
            .text()
            .await?;

        Ok(resp)
    }

    pub async fn create_devices(&self) {
        let light_count: usize = self.lights.light.lock().await.len();
        let blind_count: usize = self.blinds.blinds.lock().await.len();
        if light_count + blind_count != 0 {
            println!("Already created devices. Skipping");
            return;
        }
        let myarc = self.ui.clone();
        let mymutex = myarc.try_lock().expect("could not lock the mutex");
        let ui = mymutex.clone();
        let uii = ui.expect("Error getting ui repsonse from mutex");
        for function in uii.functions {
            let values = self.get_fn_values(function.uid.clone()).await.unwrap();
            match function.channelType.as_str() {
                "de.gira.schema.channels.Switch"
                | "de.gira.schema.channels.DimmerWhite"
                | "de.gira.schema.channels.KNX.Dimmer" => {
                    let mut myswitch_option: Option<Switch> = None;
                    let mut mydimm_option: Option<Dimmer> = None;
                    let mut mytuner_option: Option<Tuner> = None;
                    let mycolor_option: Option<Color> = None;

                    let light_type = match function.channelType.as_str() {
                        "de.gira.schema.channels.Switch" => LightType::SWITCH,
                        "de.gira.schema.channels.DimmerWhite" => LightType::TUNE,
                        "de.gira.schema.channels.KNX.Dimmer" => LightType::DIMM,
                        _ => LightType::UNKNOWN,
                    };

                    for (pindex, point) in function.dataPoints.iter().enumerate() {
                        match point.name.as_str() {
                            "OnOff" => {
                                let myswitch = Switch {
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: values
                                        .get(function.dataPoints[pindex].uid.as_str())
                                        .unwrap()
                                        .to_owned(),
                                };
                                myswitch_option = Some(myswitch)
                            }
                            "Brightness" => {
                                let mydimm = Dimmer {
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: values
                                        .get(function.dataPoints[pindex].uid.as_str())
                                        .unwrap()
                                        .to_owned(),
                                };
                                mydimm_option = Some(mydimm)
                            }
                            "Color-Temperature" => {
                                let mytuner = Tuner {
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: values
                                        .get(function.dataPoints[pindex].uid.as_str())
                                        .unwrap()
                                        .to_owned(),
                                };
                                mytuner_option = Some(mytuner)
                            }
                            _ => (),
                        }
                    }
                    let mylight = Light {
                        name: function.displayName,
                        uid: function.uid.clone(),
                        lighttype: light_type,
                        switch: myswitch_option,
                        dimmer: mydimm_option,
                        tuner: mytuner_option,
                        color: mycolor_option,
                        location: None,
                    };
                    self.lights.light.lock().await.push(mylight);
                }

                "de.gira.schema.channels.BlindWithPos" => {
                    let mut mystepupdown_option: Option<StepUpDown> = None;
                    let mut myupdown_option: Option<UpDown> = None;
                    let mut myposition_option: Option<Position> = None;
                    let mut mymovement_option: Option<Movement> = None;

                    for (pindex, point) in function.dataPoints.iter().enumerate() {
                        match point.name.as_str() {
                            "Step-Up-Down" => {
                                let mystepupdown = StepUpDown {
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: values
                                        .get(function.dataPoints[pindex].uid.as_str())
                                        .unwrap()
                                        .to_owned(),
                                };
                                mystepupdown_option = Some(mystepupdown)
                            }
                            "Up-Down" => {
                                let myupdown = UpDown {
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: values
                                        .get(function.dataPoints[pindex].uid.as_str())
                                        .unwrap()
                                        .to_owned(),
                                };
                                myupdown_option = Some(myupdown)
                            }
                            "Position" => {
                                let myposition = Position {
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: values
                                        .get(function.dataPoints[pindex].uid.as_str())
                                        .unwrap()
                                        .to_owned(),
                                };
                                myposition_option = Some(myposition)
                            }
                            "Movement" => {
                                let mymovement = Movement {
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: values
                                        .get(function.dataPoints[pindex].uid.as_str())
                                        .unwrap()
                                        .to_owned(),
                                };
                                mymovement_option = Some(mymovement)
                            }
                            _ => (),
                        }
                    }

                    let myblind = Blind {
                        uid: function.uid.clone(),
                        name: function.displayName,
                        step_up_down: mystepupdown_option,
                        up_down: myupdown_option,
                        position: myposition_option,
                        movement: mymovement_option,
                        location: None,
                    };
                    self.blinds.blinds.lock().await.push(myblind);

                    println!("Added blind")
                }

                _ => (),
            }
        }
    }

    pub fn create_locations(&self) -> UiLocation {
        let mut root = UiLocation {
            id: Some(0),
            parent_location: None,
            displayName: "Home".to_string(),
            functions: Some(vec![]),
            locationType: "root".to_string(),
            locations: Some(vec![]),
        };
        root.locations = Some(
            self.ui
                .try_lock()
                .expect("Error locking UI")
                .clone()
                .expect("Error getting UI")
                .locations,
        );
        self.set_location_id(&mut root);

        root
    }

    pub fn set_location_id(&self, location: &mut UiLocation) {
        let mut last_location_id = self
            .last_location
            .try_lock()
            .expect("Error locking last_location");
        *last_location_id += 1;

        location.id = Some(*last_location_id);
        drop(last_location_id);

        if let Some(functions) = &location.functions {
            for function in functions {
                let mut mylights = self.lights.light.try_lock().expect("Error locking lights");
                for light in mylights.iter_mut() {
                    if *function == light.uid {
                        light.location = location.id;
                    }
                }
                let mut myblinds = self.blinds.blinds.try_lock().expect("Error locking lights");
                for blind in myblinds.iter_mut() {
                    if *function == blind.uid {
                        blind.location = location.id;
                    }
                }
            }
        }

        if let Some(locations) = &mut location.locations {
            for child_location in locations.iter_mut() {
                child_location.parent_location = location.id;
                self.set_location_id(child_location);
            }
        }

        let mut location_ids: Vec<u16> = vec![];
        for loc in location.locations.clone().unwrap_or(vec![]).iter() {
            if let Some(id) = loc.id {
                location_ids.push(id);
            }
        }
        let location_id_map = Location {
            id: location.id,
            parent_location: location.parent_location,
            displayName: location.displayName.clone(),
            functions: location.functions.clone(),
            locationType: location.locationType.clone(),
            locations: Some(location_ids),
        };
        self.locations
            .locations
            .try_lock()
            .expect("error locking locations")
            .insert(location.id.unwrap(), location_id_map);
    }
    pub async fn connect(&self) {
        if *self.connected.lock().await == true {
            return;
        }
        self.connect_x1().await;
        self.get_ui().await;
        self.create_devices().await;
        self.create_locations();
        *self.connected.lock().await = true;
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Function {
    channelType: String,
    dataPoints: Vec<DataPoint>,
    displayName: String,
    functionType: String,
    uid: String,
    location: Option<u16>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataPoint {
    name: String,
    uid: String,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiLocation {
    id: Option<u16>,
    parent_location: Option<u16>,
    displayName: String,
    functions: Option<Vec<String>>,
    locationType: String,
    locations: Option<Vec<UiLocation>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Trade {
    displayName: String,
    functions: Option<Vec<String>>,
    tradeType: String,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiResponse {
    functions: Vec<Function>,
    locations: Vec<UiLocation>,
    trades: Vec<Trade>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    pub values: Option<Vec<HashMap<String, String>>>,
    pub error: Option<HashMap<String, String>>,
}
