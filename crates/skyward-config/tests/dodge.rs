//! The OnePlus 13's device file, read out of ColorOS 17's own partitions.

use skyward_config::DeviceConfig;

#[test]
fn parses_dodge() {
    let cfg = DeviceConfig::parse(include_str!("../../../devices/dodge.toml")).expect("dodge.toml parses");
    assert_eq!(cfg.device.codename, "dodge");
    assert_eq!(cfg.device.brand, "OnePlus");
    assert_eq!(cfg.device.model, "OnePlus 13");
    assert!(cfg.device.ab);
    assert_eq!(cfg.device.os_version_prop, "ro.build.version.oplusrom");
    assert_eq!((cfg.display.width, cfg.display.height, cfg.display.density), (1440, 3168, 640));
    assert_eq!(cfg.modules.load, ["oplus_bsp_tp_hbp_syna_s3910"]);
    assert_eq!(cfg.firmware.as_ref().map(|f| f.partition.as_str()), Some("modem"));
    assert_eq!(cfg.partitions.mounted, ["persist"]);
    assert_eq!(cfg.services.len(), 7, "the secure world's listeners, the key and credential programs, the weaver, the secure element and boot control");
    let weaver = cfg.services.iter().find(|s| s.path.ends_with("weaver-service.nxp")).expect("the weaver is described");
    assert_eq!(weaver.ready_service.as_deref(), Some("android.hardware.weaver.IWeaver/default"));
    let wlan = cfg.wlan.as_ref().expect("the Peach chip is described");
    assert_eq!(wlan.modules, ["rfkill", "qca_cld3_peach_v2"]);
    assert_eq!(wlan.interface, "wlan0");
    let haptics = cfg.haptics.as_ref().expect("the motor is described");
    assert_eq!(haptics.resonance, "/sys/class/qcom-haptics/lra_frequency_hz");
    assert_eq!(cfg.backup.efs, ["modemst1", "modemst2", "fsg", "fsc", "persist"]);
    let stock = cfg.stock.expect("dodge is built from ColorOS' full package");
    assert_eq!(stock.version, "PJZ110_17.0.0.103(CN01)");
    assert_eq!(stock.recovery, "406ef6641961d713951f748f4b9a415d321589b8f65d788995226375586bd016");
}
