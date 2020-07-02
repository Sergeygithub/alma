pub const BOOT_PARTITION_INDEX: u8 = 1;
pub const ROOT_PARTITION_INDEX: u8 = 3;

pub static JOURNALD_CONF: &str = "
[Journal]
Storage=volatile
SystemMaxUse=16M
";

pub const BASE_PACKAGES: [&str; 9] = [
    "base",
    "linux54",
    "linux54-broadcom-wl",
    "linux-firmware",
    "grub",
    "efibootmgr",
    "intel-ucode",
    "amd-ucode",
    "networkmanager",
];

pub const AUR_DEPENDENCIES: [&str; 3] = ["base-devel", "git", "sudo"];
