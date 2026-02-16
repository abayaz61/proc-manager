/// Detailed system information collected once at startup.
/// Includes BIOS, motherboard, CPU model, OS details etc.
pub struct SystemInfoDetail {
    // OS
    pub os_name: String,
    pub os_version: String,
    pub os_long_version: String,
    pub kernel_version: String,
    pub cpu_arch: String,

    // CPU
    pub cpu_brand: String,
    pub cpu_vendor: String,
    pub physical_core_count: usize,

    // BIOS
    pub bios_vendor: String,
    pub bios_version: String,
    pub bios_release_date: String,

    // Motherboard
    pub board_manufacturer: String,
    pub board_product: String,

    // System
    pub system_manufacturer: String,
    pub system_product: String,
    pub system_sku: String,
    pub system_family: String,
}

impl SystemInfoDetail {
    pub fn collect() -> Self {
        let cpu_brand;
        let cpu_vendor;
        let physical_core_count;
        {
            let sys = sysinfo::System::new_with_specifics(
                sysinfo::RefreshKind::nothing()
                    .with_cpu(sysinfo::CpuRefreshKind::everything()),
            );
            let cpus = sys.cpus();
            cpu_brand = cpus.first().map(|c| c.brand().to_string()).unwrap_or_default();
            cpu_vendor = cpus.first().map(|c| c.vendor_id().to_string()).unwrap_or_default();
            physical_core_count = sys.physical_core_count().unwrap_or(0);
        }

        let (bios_vendor, bios_version, bios_release_date,
             board_manufacturer, board_product,
             system_manufacturer, system_product, system_sku, system_family) = collect_bios_info();

        Self {
            os_name: sysinfo::System::name().unwrap_or_else(|| "Unknown".into()),
            os_version: sysinfo::System::os_version().unwrap_or_else(|| "Unknown".into()),
            os_long_version: sysinfo::System::long_os_version().unwrap_or_else(|| "Unknown".into()),
            kernel_version: sysinfo::System::kernel_version().unwrap_or_else(|| "Unknown".into()),
            cpu_arch: {
                let arch = sysinfo::System::cpu_arch();
                if arch.is_empty() { "Unknown".into() } else { arch }
            },
            cpu_brand,
            cpu_vendor,
            physical_core_count,
            bios_vendor,
            bios_version,
            bios_release_date,
            board_manufacturer,
            board_product,
            system_manufacturer,
            system_product,
            system_sku,
            system_family,
        }
    }
}

#[cfg(target_os = "windows")]
fn collect_bios_info() -> (String, String, String, String, String, String, String, String, String) {
    use windows::Win32::System::Registry::{
        RegOpenKeyExW, RegQueryValueExW, HKEY_LOCAL_MACHINE, KEY_READ, REG_SZ,
    };
    use windows::core::PCWSTR;

    fn read_reg_string(hkey: windows::Win32::System::Registry::HKEY, value_name: &str) -> String {
        unsafe {
            let name_wide: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
            let mut data_type = REG_SZ;
            let mut size: u32 = 0;

            // First call to get size
            let _ = RegQueryValueExW(
                hkey,
                PCWSTR(name_wide.as_ptr()),
                None,
                Some(&mut data_type),
                None,
                Some(&mut size),
            );

            if size == 0 {
                return String::new();
            }

            let mut buffer: Vec<u8> = vec![0u8; size as usize];
            let result = RegQueryValueExW(
                hkey,
                PCWSTR(name_wide.as_ptr()),
                None,
                Some(&mut data_type),
                Some(buffer.as_mut_ptr()),
                Some(&mut size),
            );

            if result.is_err() {
                return String::new();
            }

            // Convert wide string buffer to String
            let wide: &[u16] = std::slice::from_raw_parts(
                buffer.as_ptr() as *const u16,
                (size as usize) / 2,
            );
            // Trim null terminator
            let len = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
            String::from_utf16_lossy(&wide[..len])
        }
    }

    let mut bios_vendor = String::new();
    let mut bios_version = String::new();
    let mut bios_release_date = String::new();
    let mut board_manufacturer = String::new();
    let mut board_product = String::new();
    let mut system_manufacturer = String::new();
    let mut system_product = String::new();
    let mut system_sku = String::new();
    let mut system_family = String::new();

    unsafe {
        // BIOS info from registry
        let bios_key_path: Vec<u16> = "HARDWARE\\DESCRIPTION\\System\\BIOS"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        let mut hkey = windows::Win32::System::Registry::HKEY::default();
        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(bios_key_path.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );

        if result.is_ok() {
            bios_vendor = read_reg_string(hkey, "BIOSVendor");
            bios_version = read_reg_string(hkey, "BIOSVersion");
            bios_release_date = read_reg_string(hkey, "BIOSReleaseDate");
            board_manufacturer = read_reg_string(hkey, "BaseBoardManufacturer");
            board_product = read_reg_string(hkey, "BaseBoardProduct");
            system_manufacturer = read_reg_string(hkey, "SystemManufacturer");
            system_product = read_reg_string(hkey, "SystemProductName");
            system_sku = read_reg_string(hkey, "SystemSKU");
            system_family = read_reg_string(hkey, "SystemFamily");

            let _ = windows::Win32::System::Registry::RegCloseKey(hkey);
        }
    }

    (
        bios_vendor,
        bios_version,
        bios_release_date,
        board_manufacturer,
        board_product,
        system_manufacturer,
        system_product,
        system_sku,
        system_family,
    )
}

#[cfg(not(target_os = "windows"))]
fn collect_bios_info() -> (String, String, String, String, String, String, String, String, String) {
    // On Linux, read from /sys/class/dmi/id/
    fn read_dmi(name: &str) -> String {
        std::fs::read_to_string(format!("/sys/class/dmi/id/{}", name))
            .unwrap_or_default()
            .trim()
            .to_string()
    }

    (
        read_dmi("bios_vendor"),
        read_dmi("bios_version"),
        read_dmi("bios_date"),
        read_dmi("board_vendor"),
        read_dmi("board_name"),
        read_dmi("sys_vendor"),
        read_dmi("product_name"),
        read_dmi("product_sku"),
        read_dmi("product_family"),
    )
}
