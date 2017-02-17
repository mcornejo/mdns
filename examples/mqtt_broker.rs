extern crate mdns;

use std::time::Duration;

fn main() {
    let duration = Duration::from_secs(15);

    mdns::discover("_mqtt._tcp.local", Some(duration), false, |response| {
        
        let addresses = response.records().filter_map(|record| {
            if let mdns::RecordKind::A(addr) = record.kind { Some(addr) } else { None }
        });

        for address in addresses {
            println!("found broker on {}", address);
        }


    }).expect("error while performing mqtt discovery");
}

/*
fn handle_discover(service_name: &str, duration: Option<Duration>, mut f: F) {

}
*/