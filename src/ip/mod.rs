use local_ip_address::local_ip;
use crate::file;

pub mod route;

pub fn print(port: u16) {
    match local_ip() {
        Ok(ip) => {
            let storage_path = format!("{}/file", file::get_current_working_directory().unwrap_or(String::from(".")));
            let host_url = format!("http://{}:{}/", ip.to_string(), port);

            println!(r#"
  __  __     ______
 / / / /__  / __/ /________ ___ ___ _
/ /_/ / _ \_\ \/ __/ __/ -_) _ `/  ' \
\____/ .__/___/\__/_/  \__/\_,_/_/_/_/
    /_/
            "#);
            println!("{:<8}: {}", "Storage", storage_path);
            println!("{:<8}: {}", "Host", host_url);

            println!("---------- SCAN ME! ---------");
            qr2term::print_qr(&host_url).expect("Generate QR failure");
        }
        Err(_) => {}
    }
}