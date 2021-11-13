use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,

    //#[serde(default = "bool::default")]
    //pub bot: bool,
}

#[derive(Debug, Deserialize)]
pub struct Application {
    id: String,
}
