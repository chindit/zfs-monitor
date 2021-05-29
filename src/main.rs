extern crate dotenv;
extern crate lettre;
mod zpool;
mod mailer;

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    println!("Welcome to ZFS Monitor");
    println!();
    println!("Checking zPool status");

    let pool_status = zpool::health();

    if  pool_status == "ONLINE" {
        println!("Pool is ONLINE");
    } else {
        println!("Pool is NOT online.  Current status is {}", pool_status);
        mailer::mail("WARNING: zPool unstable".to_owned(), "zPool Unstable.  Current status is ".to_owned() + &pool_status);
    }

    println!("Checking capacity");
    println!();
    println!("Capacity is {}%", zpool::capacity());

    let disk_errors: Vec<String> = zpool::io_errors();
    if disk_errors.len() == 0 {
        println!("All disks are OK");
    } else {
        println!("{} disks have errors in pool", disk_errors.len());
        disk_errors.iter().for_each(|disk| println!("{}", disk));

        mailer::mail(
            "WARNING: disk have errors in pool".to_owned(),
            "Following disks have read, write or checksum errors:\n".to_string() + &disk_errors.concat()
        );
    }
}