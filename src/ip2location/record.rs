#![allow(clippy::derive_partial_eq_without_eq)]

use serde::Serialize;
use serde_with::skip_serializing_none;
use std::{
    borrow::Cow,
    net::{IpAddr, Ipv6Addr},
};

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Country<'a> {
    pub short_name: Cow<'a, str>,
    pub long_name: Cow<'a, str>,
}

#[skip_serializing_none]
#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct LocationRecord<'a> {
    pub ip: IpAddr,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub country: Option<Country<'a>>,
    pub region: Option<Cow<'a, str>>,
    pub city: Option<Cow<'a, str>>,
    pub isp: Option<Cow<'a, str>>,
    pub domain: Option<Cow<'a, str>>,
    pub zip_code: Option<Cow<'a, str>>,
    pub time_zone: Option<Cow<'a, str>>,
    pub net_speed: Option<Cow<'a, str>>,
    pub idd_code: Option<Cow<'a, str>>,
    pub area_code: Option<Cow<'a, str>>,
    pub weather_station_code: Option<Cow<'a, str>>,
    pub weather_station_name: Option<Cow<'a, str>>,
    pub mcc: Option<Cow<'a, str>>,
    pub mnc: Option<Cow<'a, str>>,
    pub mobile_brand: Option<Cow<'a, str>>,
    pub elevation: Option<Cow<'a, str>>,
    pub usage_type: Option<Cow<'a, str>>,
    pub address_type: Option<Cow<'a, str>>,
    pub category: Option<Cow<'a, str>>,
    pub district: Option<Cow<'a, str>>,
    pub asn: Option<Cow<'a, str>>,
    pub as_name: Option<Cow<'a, str>>,
}

impl LocationRecord<'_> {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Default for LocationRecord<'_> {
    fn default() -> Self {
        LocationRecord {
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
            usage_type: None,
            address_type: None,
            category: None,
            district: None,
            asn: None,
            as_name: None,
        }
    }
}
