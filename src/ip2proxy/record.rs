use std::net::{IpAddr, Ipv6Addr};

use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Country {
    pub short_name: String,
    pub long_name: String,
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum Proxy {
    IsAnError,
    IsNotAProxy,
    IsAProxy,
    IsADataCenterIpAddress,
}

#[skip_serializing_none]
#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Record {
    pub ip: IpAddr,
    pub country: Option<Country>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub isp: Option<String>,
    pub domain: Option<String>,
    pub is_proxy: Option<Proxy>,
    pub proxy_type: Option<String>,
    pub asn: Option<String>,
    pub as_: Option<String>,
    pub last_seen: Option<String>,
    pub threat: Option<String>,
    pub provider: Option<String>,
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
            country: None,
            region: None,
            city: None,
            isp: None,
            domain: None,
            is_proxy: Some(Proxy::IsAnError),
            proxy_type: None,
            asn: None,
            as_: None,
            last_seen: None,
            threat: None,
            provider: None,
            usage_type: None,
        }
    }
}
