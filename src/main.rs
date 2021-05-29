mod zpool;

fn main() {
    println!("Welcome to ZFS Monitor");
    println!();
    println!("Checking zPool status");

    let pool_status = zpool::health();

    if  pool_status == "ONLINE" {
        println!("Pool is ONLINE")
    } else {
        println!("Pool is NOT online.  Current status is {}", pool_status);
    }

    println!("Checking capacity");
    println!();
    println!("Capacity is {}%", zpool::capacity());
    //println!("{}", zpool::io_errors());
    let disk_errors: Vec<String> = zpool::io_errors();
    if disk_errors.len() == 0 {
        println!("All disks are OK");
    } else {
        println!("{} disks have errors in pool", disk_errors.len());
        disk_errors.iter().for_each(|disk| println!("{}", disk));
    }
}