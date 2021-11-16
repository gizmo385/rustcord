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
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct UnavailableGuild {
    pub id: String
}

#[derive(Debug, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: i64,
    pub hoist: bool,
    pub position: i64,
    pub permissions: i64,
    pub permissions_new: String,
    pub managed: bool,
    pub mentionable: bool
}

#[derive(Debug, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub roles: Vec<Role>
}
