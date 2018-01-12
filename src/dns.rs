/* Pi-hole: A black hole for Internet advertisements
*  (c) 2018 Pi-hole, LLC (https://pi-hole.net)
*  Network-wide ad blocking via your own hardware.
*
*  API
*  DNS API Endpoints
*
*  This file is copyright under the latest version of the EUPL.
*  Please see LICENSE file for your rights under this license. */

use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fs::File;

use util;

/// Read in a value from setupVars.conf
fn read_setup_vars(entry: &str) -> io::Result<Option<String>> {
    let file = File::open("/etc/pihole/setupVars.conf")?;
    let reader = BufReader::new(file);

    // Check every line for the key
    for line in reader.lines().filter_map(|item| item.ok()) {
        if line.contains(entry) {
            return Ok(
                // Get the right hand side if it exists and is not empty
                line.split("=")
                    .nth(1)
                    .and_then(|item| if item.len() == 0 { None } else { Some(item) })
                    .map(|item| item.to_owned())
            )
        }
    }

    Ok(None)
}

fn get_domains(file_name: &str) -> util::Reply {
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                // If the file is not found, then the list is empty. [0; 0] is an empty list of
                // type i32. We can't use [] because the type needs to be known.
                return util::reply_data([0; 0]);
            } else {
                return Err(e.into());
            }
        }
    };
    let reader = BufReader::new(file);
    let mut skip_lines = false;

    let is_wildcard = file_name == "/etc/dnsmasq.d/03-pihole-wildcard.conf";

    if is_wildcard {
        // Check if both IPv4 and IPv6 are used.
        // If so, skip every other line if we're getting wildcard domains.
        let ipv4 = read_setup_vars("IPV4_ADDRESS")?;
        let ipv6 = read_setup_vars("IPV6_ADDRESS")?;

        skip_lines = ipv4.is_some() && ipv6.is_some();
    }

    let mut skip = true;
    let mut lines = Vec::new();
    for line in reader.lines().filter_map(|item| item.ok()) {
        // Skip empty lines
        if line.len() == 0 {
            continue;
        }

        // Wildcard skip every other, see above
        if skip_lines {
            skip = !skip;

            if skip {
                continue;
            }
        }

        // Parse the line
        let mut parsed_line = if is_wildcard {
            // If we're reading wildcards, the domain is between two forward slashes
            match line.split("/").nth(1) {
                Some(s) => s.to_owned(),
                None => continue
            }
        } else {
            line
        };

        lines.push(parsed_line);
    }

    util::reply_data(lines)
}

#[get("/dns/whitelist")]
pub fn get_whitelist() -> util::Reply {
    get_domains("/etc/pihole/whitelist.txt")
}

#[get("/dns/blacklist")]
pub fn get_blacklist() -> util::Reply {
    get_domains("/etc/pihole/blacklist.txt")
}

#[get("/dns/wildlist")]
pub fn get_wildlist() -> util::Reply {
    get_domains("/etc/dnsmasq.d/03-pihole-wildcard.conf")
}

#[get("/dns/status")]
pub fn status() -> util::Reply {
    let file = File::open("/etc/dnsmasq.d/01-pihole.conf");

    let status = if file.is_err() {
        "unknown"
    } else {
        let mut buffer = String::new();
        file?.read_to_string(&mut buffer)?;

        let disabled = buffer.lines()
            .filter(|line| *line == "#addn-hosts=/etc/pihole/gravity.list")
            .count();

        match disabled {
            0 => "enabled",
            1 => "disabled",
            _ => "unknown"
        }
    };

    util::reply_data(json!({
        "status": status
    }))
}
