/* Pi-hole: A black hole for Internet advertisements
*  (c) 2018 Pi-hole, LLC (https://pi-hole.net)
*  Network-wide ad blocking via your own hardware.
*
*  API
*  Endpoints for reading domain lists
*
*  This file is copyright under the latest version of the EUPL.
*  Please see LICENSE file for your rights under this license. */

use config::Config;
use dns::list::{List, get_list};
use rocket::State;
use util;

/// Get the Whitelist domains
#[get("/dns/whitelist")]
pub fn get_whitelist(config: State<Config>) -> util::Reply {
    get_list(List::Whitelist, &config)
}

/// Get the Blacklist domains
#[get("/dns/blacklist")]
pub fn get_blacklist(config: State<Config>) -> util::Reply {
    get_list(List::Blacklist, &config)
}

/// Get the Wildcard list domains
#[get("/dns/wildlist")]
pub fn get_wildlist(config: State<Config>) -> util::Reply {
    get_list(List::Wildlist, &config)
}

#[cfg(test)]
mod test {
    use testing::test_endpoint;
    use config::PiholeFile;
    use std::collections::HashMap;
    use rocket::http::Method;

    #[test]
    fn test_get_whitelist() {
        let whitelist = ["example.com", "example.net"].join("\n");
        let setup_vars = "IPV4_ADDRESS=10.1.1.1";

        let mut data = HashMap::new();
        data.insert(PiholeFile::Whitelist, whitelist.into_bytes());
        data.insert(PiholeFile::SetupVars, setup_vars.into());

        test_endpoint(
            Method::Get,
            "/admin/api/dns/whitelist",
            HashMap::default(),
            data,
            json!({
                "data": [
                    "example.com",
                    "example.net"
                ],
                "errors": []
            })
        );
    }


    #[test]
    fn test_get_blacklist() {
        let blacklist = ["example.com", "example.net"].join("\n");
        let setup_vars = "IPV4_ADDRESS=10.1.1.1";

        let mut data = HashMap::new();
        data.insert(PiholeFile::Blacklist, blacklist.into_bytes());
        data.insert(PiholeFile::SetupVars, setup_vars.into());

        test_endpoint(
            Method::Get,
            "/admin/api/dns/blacklist",
            HashMap::default(),
            data,
            json!({
                "data": [
                    "example.com",
                    "example.net"
                ],
                "errors": []
            })
        );
    }

    #[test]
    fn test_get_wildlist() {
        let wildlist = [
            "address=/example.com/10.1.1.1",
            "address=/example.net/10.1.1.1"
        ].join("\n");
        let setup_vars = "IPV4_ADDRESS=10.1.1.1";

        let mut data = HashMap::new();
        data.insert(PiholeFile::Wildlist, wildlist.into_bytes());
        data.insert(PiholeFile::SetupVars, setup_vars.into());

        test_endpoint(
            Method::Get,
            "/admin/api/dns/wildlist",
            HashMap::default(),
            data,
            json!({
                "data": [
                    "example.com",
                    "example.net"
                ],
                "errors": []
            })
        );
    }
}
