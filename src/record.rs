use std::net::{IpAddr, Ipv6Addr};

use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Country {
    pub short_name: String,
    pub long_name: String,
}

#[skip_serializing_none]
#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Record {
    pub ip: IpAddr,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub country: Option<Country>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub isp: Option<String>,
    pub domain: Option<String>,
    pub zip_code: Option<String>,
    pub time_zone: Option<String>,
    pub net_speed: Option<String>,
    pub idd_code: Option<String>,
    pub area_code: Option<String>,
    pub weather_station_code: Option<String>,
    pub weather_station_name: Option<String>,
    pub mcc: Option<String>,
    pub mnc: Option<String>,
    pub mobile_brand: Option<String>,
    pub elevation: Option<String>,
    pub usage_type: Option<String>,
}

impl Record {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Default for Record {
    fn default() -> Self {
        Record {
            ip: IpAddr::V6(Ipv6Addr::UNSPECIFIED),
            latitude: None,
            longitude: None,
            country: None,
            region: None,
            city: None,
            isp: None,
            domain: None,
            zip_code: None,
            time_zone: None,
            net_speed: None,
            idd_code: None,
            area_code: None,
            weather_station_code: None,
            weather_station_name: None,
            mcc: None,
            mnc: None,
            mobile_brand: None,
            elevation: None,
            usage_type: None
        }
    }
}
