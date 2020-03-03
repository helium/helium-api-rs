use helium_api::Client;

fn main() {
    let client = Client::default();
    let hotspot = client.get_hotspot("11VKaN7fEvDm6NaGhcZtNSU1KAQQmTSwuuJsYYEqzh8mSWkoEUd");
    println!("Hotspot: {:?}", hotspot);
    let hotspots = client.get_hotspots("13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH");
    println!("Hotspots: {:?}", hotspots);
}
