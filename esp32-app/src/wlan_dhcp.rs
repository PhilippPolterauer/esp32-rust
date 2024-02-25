#![no_std]
#![no_main]

pub use esp32_hal as hal;
pub type BootButton = crate::hal::gpio::Gpio0<crate::hal::gpio::Input<crate::hal::gpio::PullDown>>;
pub const SOC_NAME: &str = "ESP32";

use embedded_io::*;
use embedded_svc::ipv4;
use embedded_svc::ipv4::Interface;
use embedded_svc::wifi::{AccessPointInfo, AuthMethod, ClientConfiguration, Configuration, Wifi};
use esp_backtrace as _;
use esp_println::logger::init_logger_from_env;
use esp_println::println;
use esp_wifi::initialize;
use esp_wifi::wifi::WifiStaDevice;
use esp_wifi::wifi::{utils::create_network_interface, WifiError};
use esp_wifi::wifi_interface::WifiStack;
use esp_wifi::{current_millis, EspWifiInitFor};
use hal::clock::ClockControl;
use hal::{peripherals::Peripherals, prelude::*};
use hal::{Delay, Rng};
use smoltcp::iface::SocketStorage;
use smoltcp::wire::{IpAddress, Ipv4Address};

const SSID: &str = env!("SSID");
const PASS: &str = env!("PASSWORD");


const STATIC_IP: &str = "192.168.0.33";
const GATEWAY_IP: &str = "192.168.0.1";

#[entry]
fn main() -> ! {
    init_logger_from_env();

    println!("Running test");
    println!("[RUN esp32 open_access_point]");

    let peripherals = Peripherals::take();

    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);
    let timer = hal::timer::TimerGroup::new(peripherals.TIMG1, &clocks).timer0;
    let init = initialize(
        EspWifiInitFor::Wifi,
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();

    let wifi = peripherals.WIFI;
    let mut socket_set_entries: [SocketStorage; 3] = Default::default();
    let (iface, device, mut controller, sockets) =
        create_network_interface(&init, wifi, WifiStaDevice, &mut socket_set_entries).unwrap();
    let mut wifi_stack = WifiStack::new(iface, device, sockets, current_millis);

    let client_config = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        password: PASS.try_into().unwrap(),
        ..Default::default()
    });
    let res = controller.set_configuration(&client_config);
    println!("wifi_set_configuration returned {:?}", res);
    wifi_stack.work();
    
    controller.start().unwrap();
    
    println!("is wifi started: {:?}", controller.is_started());
    println!("is STA enabled: {:?}", controller.is_sta_enabled());
    println!("{:?}", controller.get_capabilities());

    let mut tries = 15;
    'tryconnection: loop {
        'outer: loop {
            println!("Start Wifi Scan");
            let res: Result<(heapless::Vec<AccessPointInfo, 10>, usize), WifiError> =
                controller.scan_n();
            if let Ok((res, _count)) = res {
                for ap in res {
                    println!("{:?}", ap);
                    if ap.ssid == SSID {
                        break 'outer;
                    }
                }
            }
            tries -= 1;
            if tries == 0 {
                break 'outer;
            }

            let wait_end = current_millis() + 1 * 1000;
            while current_millis() < wait_end {}
        }

        println!("wifi_connect {:?}", controller.connect());

        // wait to get connected
        println!("Wait to get connected");
        loop {
            let res = controller.is_connected();
            match res {
                Ok(connected) => {
                    if connected {
                        break 'tryconnection;
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                    break;
                }
            }
        }

        if let Ok(c) = controller.is_connected() {
            if !c {
                println!("[FAILED]");
                loop {}
            }
        }
    }

    println!("Setting static IP {}", STATIC_IP);
    let esp_ip = ipv4::Ipv4Addr::from(parse_ip(STATIC_IP));
    let use_dhcp = true;
    if use_dhcp {
        wifi_stack
            .set_iface_configuration(&ipv4::Configuration::Client(
                ipv4::ClientConfiguration::DHCP(ipv4::DHCPClientSettings {
                    hostname: Some("esp32".try_into().unwrap()),
                }),
            ))
            .unwrap();
    } else {
        wifi_stack
            .set_iface_configuration(&ipv4::Configuration::Client(
                ipv4::ClientConfiguration::Fixed(ipv4::ClientSettings {
                    ip: esp_ip,
                    subnet: ipv4::Subnet {
                        gateway: ipv4::Ipv4Addr::from(parse_ip(GATEWAY_IP)),
                        mask: ipv4::Mask(24),
                    },
                    dns: None,
                    secondary_dns: None,
                }),
            ))
            .unwrap();
    }

    loop {
        if wifi_stack.is_iface_up() {
            wifi_stack.get_ip_addresses(|ip| println!("IP: {:?}", ip));
            break;
        }
        wifi_stack.get_ip_addresses(|ip| println!("IP: {:?}", ip));
        println!("waiting for iface to be up");
        wifi_stack.work();
        println!("is wifi connected: {:?}", controller.is_connected());
        delay.delay_ms(500u32);
    }

    let mut rx_buffer = [0u8; 1536];
    let mut tx_buffer = [0u8; 1536];
    let mut socket = wifi_stack.get_socket(&mut rx_buffer, &mut tx_buffer);
    socket.listen(8080).unwrap();

    loop {
        socket.work();

        if !socket.is_open() {
            socket.listen(8080).unwrap();
        }

        if socket.is_connected() {
            println!("Connected");
            socket.write_all(b"DATA!").unwrap();
            socket.flush().unwrap();
            socket.close();
            println!("Done\n");
            println!();
        }
    }

    'outer: loop {
        socket.work();

        let [a, b, c, d] = parse_ip(STATIC_IP);
        socket
            .open(IpAddress::Ipv4(Ipv4Address::new(a, b, c, d)), 8080)
            .unwrap();

        loop {
            let mut buffer = [0u8; 512];
            if let Ok(len) = socket.read(&mut buffer) {
                let to_print = unsafe { core::str::from_utf8_unchecked(&buffer[..len]) };
                println!("{}", to_print);
                if to_print.contains("DATA") {
                    println!("[PASSED]");
                    break 'outer;
                }
            } else {
                break;
            }
        }
        println!();

        socket.disconnect();
    }

    loop {}
}

fn parse_ip(ip: &str) -> [u8; 4] {
    let mut result = [0u8; 4];
    for (idx, octet) in ip.split(".").into_iter().enumerate() {
        result[idx] = u8::from_str_radix(octet, 10).unwrap();
    }
    result
}
