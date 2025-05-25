use crate::covers::Blind;
use crate::covers::Blinds;
use crate::covers::Movement;
use crate::covers::Position;
use crate::covers::StepUpDown;
use crate::covers::UpDown;
use crate::lights::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub struct X1<'a> {
    addr: String,
    user: String,
    password: String,
    client: reqwest::Client,
    token: Arc<Mutex<Option<String>>>,
    ui: Arc<Mutex<Option<UiResponse>>>,
    pub lights: Lights<'a>,
    pub blinds: Blinds<'a>,
}

impl<'a> X1<'a> {
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
            lights: Lights::new(),
            blinds: Blinds::new(),
        }
    }

    pub async fn connect(&'a self) {
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
        let mut mymutex = myarc.lock().expect("could not lock the mutex");
        *mymutex = Some(token.to_owned());

        //self.get_ui().await;
        //self.create_lights();
    }

    pub fn get_token(&self) -> Option<String> {
        let myarc = self.token.clone();
        let mymutex = myarc.lock().expect("could not lock the mutex");
        mymutex.clone()
    }

    pub async fn get_ui(&self) -> String {
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
        let mut mymutex = myarc.lock().expect("could not lock the mutex");
        *mymutex = Some(myresp);
        resp
    }

    pub async fn get_value(&self, uid: String) -> Result<String, reqwest::Error> {
        let token = self
            .get_token()
            .expect("Error geting token. Not logged in?");
        let addr = self.addr.clone();
        let resp = self
            .client
            .get(format!("https://{addr}/api/v2/values/{uid}?token={token}"))
            .send()
            .await?
            .text()
            .await?;
        Ok(resp)
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

    pub fn create_devices(&'a self) {
        let myarc = self.ui.clone();
        let mymutex = myarc.lock().expect("could not lock the mutex");
        let ui = mymutex.clone();
        let uii = ui.expect("Error getting ui repsonse from mutex");
        for function in uii.functions {
            match function.channelType.as_str() {
                "de.gira.schema.channels.DimmerWhite" => {
                    let myswitch = Switch {
                        x1: &self,
                        uid: function.dataPoints[0].uid.clone(),
                        val: 0,
                    };
                    let mydimmer = Dimm {
                        x1: &self,
                        uid: function.dataPoints[1].uid.clone(),
                        val: 0,
                    };
                    let mytuner = ColorTemp {
                        x1: &self,
                        uid: function.dataPoints[2].uid.clone(),
                        val: 0,
                    };
                    let mylight = TunableLight {
                        x1: &self,
                        name: function.displayName,
                        switch: myswitch,
                        dimmer: mydimmer,
                        tuner: mytuner,
                    };
                    self.lights.tunable.add(mylight);
                    //self.lights.lock().unwrap().tunable.push(mylight);
                    //lights.tunable.push(mylight);
                    //println!("Added light")
                }
                "de.gira.schema.channels.Switch" => {
                    let mut myswitch_option: Option<Switch> = None;

                    for (pindex, point) in function.dataPoints.iter().enumerate() {
                        match point.name.as_str() {
                            "OnOff" => {
                                let myswitch = Switch {
                                    x1: &self,
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: 0,
                                };
                                myswitch_option = Some(myswitch)
                            }
                            _ => (),
                        }
                    }
                    let mylight = SwitchedLight {
                        x1: &self,
                        name: function.displayName,
                        switch: myswitch_option,
                    };
                    self.lights.switchable.add(mylight);
                }
                "de.gira.schema.channels.KNX.Dimmer" => {
                    let mut myswitch_option: Option<Switch> = None;
                    let mut mydimm_option: Option<Dimm> = None;

                    for (pindex, point) in function.dataPoints.iter().enumerate() {
                        match point.name.as_str() {
                            "OnOff" => {
                                let myswitch = Switch {
                                    x1: &self,
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: 0,
                                };
                                myswitch_option = Some(myswitch)
                            }
                            "Brightness" => {
                                let mydimm = Dimm {
                                    x1: &self,
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: 0,
                                };
                                mydimm_option = Some(mydimm)
                            }
                            _ => (),
                        }
                    }
                    let mylight = DimmedLight {
                        x1: &self,
                        name: function.displayName,
                        switch: myswitch_option,
                        dimmer: mydimm_option,
                    };
                    self.lights.dimmable.add(mylight);
                }
                "de.gira.schema.channels.BlindWithPos" => {
                    let mut mystepupdown_option: Option<StepUpDown<'_>> = None;
                    let mut myupdown_option: Option<UpDown<'_>> = None;
                    let mut myposition_option: Option<Position<'_>> = None;
                    let mut mymovement_option: Option<Movement<'_>> = None;
                    for (pindex, point) in function.dataPoints.iter().enumerate() {
                        match point.name.as_str() {
                            "Step-Up-Down" => {
                                let mystepupdown = StepUpDown {
                                    x1: &self,
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: 0,
                                };
                                mystepupdown_option = Some(mystepupdown)
                            }
                            "Up-Down" => {
                                let myupdown = UpDown {
                                    x1: &self,
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: 0,
                                };
                                myupdown_option = Some(myupdown)
                            }
                            "Position" => {
                                let myposition = Position {
                                    x1: &self,
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: 0,
                                };
                                myposition_option = Some(myposition)
                            }
                            "Movement" => {
                                let mymovement = Movement {
                                    x1: &self,
                                    uid: function.dataPoints[pindex].uid.clone(),
                                    val: 0,
                                };
                                mymovement_option = Some(mymovement)
                            }
                            _ => (),
                        }
                    }

                    let myblind = Blind {
                        x1: &self,
                        name: function.displayName,
                        step_up_down: mystepupdown_option,
                        up_down: myupdown_option,
                        position: myposition_option,
                        movement: mymovement_option,
                    };
                    self.blinds.add(myblind);

                    println!("Added blind")
                }
                _ => (),
            }
        }
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
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataPoint {
    name: String,
    uid: String,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Location {
    displayName: String,
    functions: Option<Vec<String>>,
    locationType: String,
    locations: Option<Vec<Location>>,
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
    locations: Vec<Location>,
    trades: Vec<Trade>,
}
