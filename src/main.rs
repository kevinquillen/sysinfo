#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::SharedString;
use sysinfo::{Disks, System};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    let mut system = System::new_all();
    system.refresh_all();

    let os = System::name().unwrap();
    let dist = System::distribution_id();
    let version = System::long_os_version().unwrap();
    let total_memory = system.total_memory() / (1024 * 1024 * 1024);

    let disks = Disks::new_with_refreshed_list();


    main_window.set_os(SharedString::from(format!("{}", os)));
    main_window.set_dist(SharedString::from(format!("{}", dist)));
    main_window.set_version(SharedString::from(format!("{}", version)));
    main_window.set_memory(SharedString::from(format!("{}", total_memory)));
    main_window.set_cpu(SharedString::from(format!("{}", System::cpu_arch().unwrap())));

    for disk in disks.list() {
        main_window.set_disk(SharedString::from(format!("{}", disk.available_space() / 1_000_000_000)));
        main_window.set_disk_size(SharedString::from(format!("{}", disk.total_space() / 1_000_000_000)));
        break;
    }

    main_window.run()?;
    Ok(())
}