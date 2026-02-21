use std::{error::Error, fmt, fs, io, net::{IpAddr, Ipv4Addr, Ipv6Addr}, path::Path, str::FromStr};
use serde::{Deserialize, Serialize, de};


#[derive(Serialize, Deserialize)]
pub struct Toast {
    pub r#type: ToastType,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub enum ToastType {
    Danger,
    Warning,
    Info,
    Success,
    None
}

impl fmt::Display for ToastType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Danger => "Danger",
            Self::Warning => "Warning",
            Self::Info => "Info",
            Self::Success => "Success",
            Self::None => "None"
        })?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Modal {
    pub title: String,
    pub r#type: ModalType,
    pub data: ModalData,
    pub plugin: Option<String>
}

#[derive(Serialize, Deserialize)]
pub enum ModalType {
    PluginForm
}

#[derive(Serialize, Deserialize)]
pub enum ModalData {
    PluginForm { config: Vec<Vec<PluginFormField>> }
}

#[derive(Serialize, Deserialize)]
pub struct PluginFormField {
    name: String,
    title: String,
    r#type: String,
    options: Option<Vec<String>>,
    min: Option<String>,
    max: Option<String>,
    step: Option<String>,
    regex: Option<String>,
    default: Option<String>
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct IpCidr {
    pub ip: IpAddr,
    pub mask: u8
}

impl IpCidr {
    pub fn network_size(&self) -> u128 {
        match self.ip {
            IpAddr::V4(_) => 2_u128.pow(32 - self.mask as u32),
            IpAddr::V6(_) => 2_u128.pow(128 - self.mask as u32)
        }
    }
    
    pub fn octects(&self) -> Vec<u8> {
        match self.ip {
            IpAddr::V4(ip) => {
                let mut a: Vec<u8> = Vec::new();
                a.extend_from_slice(&ip.octets());
                a.push(self.mask);
                
                a
            },
            IpAddr::V6(ip) => {
                let mut a: Vec<u8> = Vec::new();
                a.extend_from_slice(&ip.octets());
                a.push(self.mask);
                
                a
            }
        }
    }
}

impl From<[u8; 5]> for IpCidr {
    fn from(value: [u8; 5]) -> Self {
        Self {
            ip: IpAddr::V4(Ipv4Addr::from_octets(value[..4].try_into().unwrap())),
            mask: value[4]
        }
    }
}

impl From<[u8; 17]> for IpCidr {
    fn from(value: [u8; 17]) -> Self {
        Self {
            ip: IpAddr::V6(Ipv6Addr::from_octets(value[..16].try_into().unwrap())),
            mask: value[16]
        }
    }
}

impl fmt::Display for IpCidr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.ip, self.mask)
    }
}

#[derive(Debug)]
pub enum ParseIpCidrErr {
    InvalidCidr,
    InvalidIp,
    InvalidSubnetMask
}

impl ParseIpCidrErr {
    pub fn description(&self) -> &str {
        match *self {
            ParseIpCidrErr::InvalidCidr => "Invalid CIDR notation.",
            ParseIpCidrErr::InvalidIp => "The IP part is invalid.",
            ParseIpCidrErr::InvalidSubnetMask => "The subnet mask part is invalid."
        }
    }
}

impl Error for ParseIpCidrErr {}

impl fmt::Display for ParseIpCidrErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl FromStr for IpCidr {
    type Err = ParseIpCidrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split('/');
        let ip_s = splits.next();
        let mask_s = splits.next();
        
        if ip_s.is_some() {
            match ip_s.unwrap().parse::<IpAddr>() {
                Ok(ip) => {
                    if mask_s.is_some() {
                        match mask_s.unwrap().parse::<u8>() {
                            Ok(mask) => {
                                if (ip.is_ipv4() && mask <= 32) || (ip.is_ipv6() && mask <= 128) {
                                    Ok(Self {
                                        ip,
                                        mask
                                    })
                                }
                                else {
                                    Err(ParseIpCidrErr::InvalidSubnetMask)
                                }
                            },
                            Err(_) => Err(ParseIpCidrErr::InvalidSubnetMask)
                        }
                    }
                    else {
                        Ok(Self {
                            ip,
                            mask: match ip {
                                IpAddr::V4(_) => 32,
                                IpAddr::V6(_) => 128
                            }
                        })
                    }
                },
                Err(_) => Err(ParseIpCidrErr::InvalidIp)
            }
        }
        else {
            Err(ParseIpCidrErr::InvalidCidr)
        }
    }
}

impl Serialize for IpCidr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        if serializer.is_human_readable() {
            serializer.collect_str(self)
        }
        else {
            serializer.serialize_bytes(&self.octects())
        }
    }
}

impl<'de> Deserialize<'de> for IpCidr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de> {
        struct IpCidrVisitor;
        impl<'de> de::Visitor<'de> for IpCidrVisitor {
            type Value = IpCidr;
            
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: de::Error, {
                v.parse().map_err(|err| E::custom(err))
            }
            
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where
                    E: de::Error, {
                if v.len() == 5 {
                    Ok(IpCidr {
                        ip: IpAddr::V4(Ipv4Addr::from_octets(v[..4].try_into().unwrap())),
                        mask: v[4]
                    })
                }
                else if v.len() == 17 {
                    Ok(IpCidr {
                        ip: IpAddr::V4(Ipv4Addr::from_octets(v[..16].try_into().unwrap())),
                        mask: v[16]
                    })
                }
                else {
                    Err(E::invalid_length(v.len(), &self))
                }
            }
            
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "either a string representation of a CIDR notation or a 5 or 17 element byte array"
                )
            }
        }
        
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(IpCidrVisitor)
        }
        else {
            deserializer.deserialize_bytes(IpCidrVisitor)
        }
    }
}


/// MAC address implmentation based on: https://docs.rs/pnet/latest/pnet/datalink/struct.MacAddr.html
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MacAddr(pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);

impl MacAddr {
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        Self(a, b, c, d, e, f)
    }
    
    pub fn zero() -> Self {
        // Same as `Default::default()`
        Self(0, 0, 0, 0, 0, 0)
    }
    
    pub fn broadcast() -> Self {
        [0xff; 6].into()
    }
    
    pub fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
    
    pub fn is_local(&self) -> bool {
        (self.0 & 2) == 2
    }
    
    pub fn is_universal(&self) -> bool {
        !self.is_local()
    }
    
    pub fn is_multicast(&self) -> bool {
        (self.0 & 1) == 1
    }
    
    pub fn is_unicast(&self) -> bool {
        !self.is_multicast()
    }
    
    pub fn is_broadcast(&self) -> bool {
        *self == Self::broadcast()
    }
    
    pub fn octets(&self) -> [u8; 6] {
        [self.0, self.1, self.2, self.3, self.4, self.5]
    }
}

impl From<[u8; 6]> for MacAddr {
    fn from(value: [u8; 6]) -> Self {
        Self(value[0], value[1], value[2], value[3], value[4], value[5])
    }
}

#[derive(Debug)]
pub enum ParseMacAddrErr {
    TooManyComponents,
    TooFewComponents,
    InvalidComponent
}

impl ParseMacAddrErr {
    pub fn description(&self) -> &str {
        match *self {
            ParseMacAddrErr::TooManyComponents => "Too many components in a MAC address string",
            ParseMacAddrErr::TooFewComponents => "Too few components in a MAC address string",
            ParseMacAddrErr::InvalidComponent => "Invalid component in a MAC address string",
        }
    }
}

impl Error for ParseMacAddrErr {}

impl fmt::Display for ParseMacAddrErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl FromStr for MacAddr {
    type Err = ParseMacAddrErr;
    fn from_str(s: &str) -> Result<Self, ParseMacAddrErr> {
        let mut parts = [0u8; 6];
        let splits = s.split(':');
        let mut i = 0;
        for split in splits {
            if i == 6 {
                return Err(ParseMacAddrErr::TooManyComponents);
            }
            match u8::from_str_radix(split, 16) {
                Ok(b) if split.len() != 0 => parts[i] = b,
                _ => return Err(ParseMacAddrErr::InvalidComponent)
            }
            i += 1;
        }
        
        if i == 6 {
            Ok(Self::from(parts))
        }
        else {
            Err(ParseMacAddrErr::TooFewComponents)
        }
    }
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0, self.1, self.2, self.3, self.4, self.5
        )
    }
}

impl Serialize for MacAddr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        if serializer.is_human_readable() {
            serializer.collect_str(self)
        }
        else {
            serializer.serialize_bytes(&self.octets())
        }
    }
}

impl<'de> Deserialize<'de> for MacAddr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        struct MacAddrVisitor;
        impl <'de> de::Visitor<'de> for MacAddrVisitor {
            type Value = MacAddr;
            
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: de::Error, {
                v.parse().map_err(|err| E::custom(err))
            }
            
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where
                    E: de::Error, {
                if v.len() == 6 {
                    Ok(MacAddr::new(v[0], v[1], v[2], v[3], v[4], v[5]))
                }
                else {
                    Err(E::invalid_length(v.len(), &self))
                }
            }
            
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "either a string representation of a MAC address or 6-element byte array"
                )
            }
        }
        
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(MacAddrVisitor)
        }
        else {
            deserializer.deserialize_bytes(MacAddrVisitor)
        }
    }
}