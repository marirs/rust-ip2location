#![allow(clippy::enum_variant_names, clippy::derive_partial_eq_without_eq)]

use serde::Serialize;
use serde_with::skip_serializing_none;
use std::{
    borrow::Cow,
    net::{IpAddr, Ipv6Addr},
};

/// ISO 3166 country information (proxy database variant).
#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Country<'a> {
    /// Two-letter country code (e.g. `"US"`).
    pub short_name: Cow<'a, str>,
    /// Full country name (e.g. `"United States of America"`).
    pub long_name: Cow<'a, str>,
}

/// Proxy classification for an IP address.
#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum Proxy {
    /// Lookup failed or record is in an error state.
    IsAnError,
    /// The IP is not a known proxy.
    IsNotAProxy,
    /// The IP is a known proxy (VPN, open proxy, etc.).
    IsAProxy,
    /// The IP belongs to a data center / hosting provider.
    IsADataCenterIpAddress,
}

/// Proxy detection record returned by an IP2Proxy database lookup.
///
/// Which fields are populated depends on the database type (PX1–PX11).
/// Unpopulated fields are `None` and omitted during JSON serialisation.
/// String fields borrow from the memory-mapped file (zero-copy).
#[skip_serializing_none]
#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct ProxyRecord<'a> {
    pub ip: IpAddr,
    pub country: Option<Country<'a>>,
    pub region: Option<Cow<'a, str>>,
    pub city: Option<Cow<'a, str>>,
    pub isp: Option<Cow<'a, str>>,
    pub domain: Option<Cow<'a, str>>,
    pub is_proxy: Option<Proxy>,
    pub proxy_type: Option<Cow<'a, str>>,
    pub asn: Option<Cow<'a, str>>,
    pub as_: Option<Cow<'a, str>>,
    pub last_seen: Option<Cow<'a, str>>,
    pub threat: Option<Cow<'a, str>>,
    pub provider: Option<Cow<'a, str>>,
    pub usage_type: Option<Cow<'a, str>>,
}

impl ProxyRecord<'_> {
    /// Serialise this record to a JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Default for ProxyRecord<'_> {
    fn default() -> Self {
        ProxyRecord {
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
