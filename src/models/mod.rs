mod appstate;
pub mod config;
mod param;
mod user;
mod error;
mod role;
mod episode;
mod ytdlp;
pub mod channel;
pub mod response;
pub mod ytinfo;


use chrono::{
    DateTime,
    Utc,
};

pub use error::Error;
pub use config::Config;
pub use param::Param;
pub use response::{
    CustomResponse,
    CResponse
};
pub use ytdlp::{
    Ytdlp,
    YtVideo
};
pub use channel::{
    Channel,
    NewChannel,
    UpdateChannel,
};
use ytinfo::YTInfo;
pub use episode::Episode;
pub use appstate::AppState;
pub use user::{
    User,
    NewUser,
    Credentials,
};

pub fn default_datetime() -> DateTime<Utc> {
    Utc::now()
}
