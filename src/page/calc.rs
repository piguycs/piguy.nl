use std::{fmt::Write, net::Ipv4Addr, str::FromStr};

use maud::html;
use worker::*;

use crate::component::{base, into_response};

pub fn subnet(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body = html! {
        form hx-post {
            input name="addr";
            input name="mask";
            input type="submit";
        }
    };

    into_response(base("Subnet Calculator", "", body, true))
}

#[derive(Debug, Clone)]
struct SubnetReq {
    addr: Ipv4Addr,
    mask: Ipv4Addr,
}

impl SubnetReq {
    fn parse_form(form: FormData) -> Option<Self> {
        let addr = form.get("addr")?;
        let mask = form.get("mask")?;

        match (addr, mask) {
            (FormEntry::Field(addr), FormEntry::Field(mask)) => {
                dbg!(&addr);
                dbg!(&mask);
                let addr = Ipv4Addr::from_str(&addr).ok()?;
                let mask = Ipv4Addr::from_str(&mask).ok()?;

                Some(Self { addr, mask })
            }
            _ => None,
        }
    }

    #[allow(unused_must_use)]
    fn print_info(&self) -> String {
        let addr_u32 = u32::from(self.addr);
        let mask_u32 = u32::from(self.mask);
        let network_u32 = addr_u32 & mask_u32;
        let wildcard_u32 = !mask_u32;
        let broadcast_u32 = network_u32 | wildcard_u32;
        let host_bits = wildcard_u32.count_ones();
        let max_hosts = if host_bits > 1 {
            (1 << host_bits) - 2
        } else {
            0
        };
        let first_host_u32 = network_u32 + 1;
        let last_host_u32 = broadcast_u32 - 1;

        // Convert to Ipv4Addr
        let network_addr = Ipv4Addr::from(network_u32);
        let broadcast_addr = Ipv4Addr::from(broadcast_u32);
        let first_host_addr = Ipv4Addr::from(first_host_u32);
        let last_host_addr = Ipv4Addr::from(last_host_u32);
        let wildcard_addr = Ipv4Addr::from(wildcard_u32);

        let mut st = String::new();

        write!(st, "Address       = {}", self.addr);
        write!(
            st,
            "               = {:08b} . {:08b} . {:08b} . {:08b}",
            (addr_u32 >> 24) & 0xFF,
            (addr_u32 >> 16) & 0xFF,
            (addr_u32 >> 8) & 0xFF,
            addr_u32 & 0xFF
        );
        write!(
            st,
            "Network       = {} / {}",
            network_addr,
            32 - mask_u32.count_ones()
        );
        write!(st, "Netmask       = {}", self.mask);
        write!(st, "Broadcast     = {}", broadcast_addr);
        write!(st, "Wildcard Mask = {}", wildcard_addr);
        write!(st, "Hosts Bits    = {}", host_bits);
        write!(st, "Max. Hosts    = {}   (2^{} - 2)", max_hosts, host_bits);
        write!(
            st,
            "Host Range    = {{ {} - {} }}",
            first_host_addr, last_host_addr
        );
        write!(st, "Properties    =");
        write!(
            st,
            "   - {} is a HOST address in {}/{}",
            self.addr,
            network_addr,
            32 - mask_u32.count_ones()
        );

        st
    }
}

pub async fn subnet_post(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let form = req.form_data().await?;
    let req_data = match SubnetReq::parse_form(form) {
        Some(req_data) => req_data,
        None => return Ok(Response::ok("malformed data")?.with_status(422)),
    };

    Response::ok(html! {
        (req_data.print_info().as_str())
    })
}
