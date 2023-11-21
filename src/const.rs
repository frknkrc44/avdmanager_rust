/*
 *  This file is part of avdmanager_rust.
 *
 *  avdmanager_rust is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  avdmanager_rust is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *   along with avdmanager_rust.  If not, see <https://www.gnu.org/licenses/>.
 */

pub const _AVD_ID_KEY: &str = "AvdId";
pub const _PLAY_STORE_ENABLED_KEY: &str = "PlayStore.enabled";
pub const _ABI_TYPE_KEY: &str = "abi.type";
pub const _AVD_DISPLAY_NAME_KEY: &str = "avd.ini.displayname";
pub const _AVD_ENCODING_KEY: &str = "avd.ini.encoding";
pub const _USERDATA_SIZE_KEY: &str = "disk.dataPartition.size";
pub const _FASTBOOT_CHOSEN_SNAPSHOT_FILE_KEY: &str = "fastboot.chosenSnapshotFile";
pub const _FASTBOOT_FORCE_CHOSEN_SNAPSHOT_BOOT_KEY: &str = "fastboot.forceChosenSnapshotBoot";
pub const _FASTBOOT_FORCE_COLD_BOOT_KEY: &str = "fastboot.forceColdBoot";
pub const _FASTBOOT_FORCE_FAST_BOOT_KEY: &str = "fastboot.forceFastBoot";
pub const _HW_ACCELEROMETER_KEY: &str = "hw.accelerometer";
pub const _HW_ARC_KEY: &str = "hw.arc";
pub const _HW_AUDIO_INPUT_KEY: &str = "hw.audioInput";
pub const _HW_BATTERY_KEY: &str = "hw.battery";
pub const _HW_CAMERA_BACK_KEY: &str = "hw.camera.back";
pub const _HW_CAMERA_FRONT_KEY: &str = "hw.camera.front";
pub const _HW_CPU_ARCH_KEY: &str = "hw.cpu.arch";
pub const _HW_CPU_NCORE_KEY: &str = "hw.cpu.ncore";
pub const _HW_DPAD_KEY: &str = "hw.dPad";
pub const _HW_DEVICE_HASH2_KEY: &str = "hw.device.hash2";
pub const _HW_DEVICE_MANUFACTURER_KEY: &str = "hw.device.manufacturer";
pub const _HW_DEVICE_NAME_KEY: &str = "hw.device.name";
pub const _HW_GPS_KEY: &str = "hw.gps";
pub const _HW_GPU_ENABLED_KEY: &str = "hw.gpu.enabled";
pub const _HW_GPU_MODE_KEY: &str = "hw.gpu.mode";
pub const _HW_INITIAL_ORIENTATION_KEY: &str = "hw.initialOrientation";
pub const _HW_KEYBOARD_KEY: &str = "hw.keyboard";
pub const _HW_LCD_DENSITY_KEY: &str = "hw.lcd.density";
pub const _HW_LCD_HEIGHT_KEY: &str = "hw.lcd.height";
pub const _HW_LCD_WIDTH_KEY: &str = "hw.lcd.width";
pub const _HW_MAIN_KEYS_KEY: &str = "hw.mainKeys";
pub const _HW_RAM_SIZE_KEY: &str = "hw.ramSize";
pub const _HW_SD_CARD_KEY: &str = "hw.sdCard";
pub const _HW_SENSORS_ORIENTATION_KEY: &str = "hw.sensors.orientation";
pub const _HW_SENSORS_PROXIMITY_KEY: &str = "hw.sensors.proximity";
pub const _HW_TRACK_BALL_KEY: &str = "hw.trackBall";
pub const _IMAGE_SYS_DIR_KEY: &str = "image.sysdir.1";
pub const _RUNTIME_NETWORK_LATENCY_KEY: &str = "runtime.network.latency";
pub const _RUNTIME_NETWORK_SPEED_KEY: &str = "runtime.network.speed";
pub const _SD_CARD_SIZE_KEY: &str = "sdcard.size";
pub const _SHOW_DEVICE_FRAME_KEY: &str = "showDeviceFrame";
pub const _SKIN_DYNAMIC_KEY: &str = "skin.dynamic";
pub const _SKIN_NAME_KEY: &str = "skin.name";
pub const _SKIN_PATH_KEY: &str = "skin.path";
pub const _SKIN_PATH_BACKUP_KEY: &str = "skin.path.backup";
pub const _TAG_DISPLAY_KEY: &str = "tag.display";
pub const _TAG_ID_KEY: &str = "tag.id";
pub const _VM_HEAP_SIZE_KEY: &str = "vm.heapSize";

pub const KIB: u64 = 1024;
pub const MIB: u64 = KIB * KIB ;
pub const GIB: u64 = MIB * KIB ;
pub const TIB: u64 = GIB * KIB ;
pub const PIB: u64 = TIB * KIB ;
