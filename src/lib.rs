#[cfg(feature = "async")]
use futures::future::Future;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

#[derive(PartialEq, Clone)]
pub enum SocketType {
    RawIpv4,
    RawIpv6,
    ICMP,
    TCP,
    UDP,
}

#[derive(Debug)]
pub enum SocketSendError {
    SocketNotReady,
    Exhausted,
    Unknown(String),
}

#[derive(Debug)]
pub enum SocketReceiveError {
    SocketNotReady,
    Unknown(String),
}

#[derive(Debug)]
pub enum SocketConnectionError {
    Exhausted,
    Unknown(String),
}

pub type OnTcpSocketData = Arc<dyn Fn(&[u8]) -> Result<usize, SocketReceiveError>>;
pub type OnUdpSocketData = Arc<dyn Fn(&[u8], IpAddr, u16) -> Result<usize, SocketReceiveError>>;

#[derive(Clone)]
pub struct UdpPacket {
    pub data: Vec<u8>,
    pub from_ip: IpAddr,
    pub from_port: u16 
}

#[derive(Clone)]
pub struct TcpPacket {
    pub data: Vec<u8>
}

#[derive(Clone)]
pub struct Packet {
    pub tcp: Option<TcpPacket>,
    pub udp: Option<UdpPacket>,
    //pub index: usize
}

//unsafe impl Sync for Packet{}


impl Packet {
    pub fn new_tcp(tcp_packet: TcpPacket) -> Packet{
        Packet {
            tcp: Some(tcp_packet),
            udp: None,
            //index: 0
        }
    }

    pub fn new_udp(udp_packet: UdpPacket) -> Packet{
        Packet {
            tcp: None,
            udp: Some(udp_packet),
            //index: 0
        }
    }
}

pub trait Socket {
    fn tcp_socket_send(&mut self, data: &[u8]) -> Result<usize, SocketSendError>;
    fn udp_socket_send(&mut self, data: &[u8], addr: SocketAddr) -> Result<usize, SocketSendError>;
    fn tcp_connect(&mut self, addr: SocketAddr, port: u16) -> Result<(), SocketConnectionError>;
    fn tcp_receive(&mut self, f: &dyn Fn(&[u8])) -> Result<usize, SocketReceiveError>;
    fn udp_receive(
        &mut self,
        f: &dyn Fn(&[u8], SocketAddr, u16),
    ) -> Result<usize, SocketReceiveError>;
}
#[cfg(feature = "async")]
pub trait AsyncSocket {
    fn tcp_socket_send<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<usize, SocketSendError>> + Send + 'a>>;
    fn tcp_connect<'a>(
        &'a mut self,
        addr: SocketAddr,
        src_port: u16,
    ) -> Pin<Box<dyn Future<Output = Result<(), SocketConnectionError>> + Send + 'a>>;
    fn tcp_receive<'a>(
        &'a mut self,
        f: &'a dyn Fn(&[u8]),
    ) -> Pin<Box<dyn Future<Output = Result<usize, SocketReceiveError>> + Send + 'a>>;
    fn udp_socket_send<'a>(
        &'a mut self,
        data: &'a [u8],
        addr: SocketAddr,
    ) -> Pin<Box<dyn Future<Output = Result<usize, SocketSendError>> + Send + 'a>>;
    fn udp_receive<'a>(
        &'a mut self,
        f: &'a dyn Fn(&[u8], SocketAddr, u16),
    ) -> Pin<Box<dyn Future<Output = Result<usize, SocketReceiveError>> + Send + 'a>>;
}
