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

#![allow(dead_code)]
#![allow(unreachable_patterns)]

use std::collections::LinkedList;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::ReadDir;
use std::io::Read;
use std::path::Path;

const _AVD_ID_KEY: &str = "AvdId";
const _PLAY_STORE_ENABLED_KEY: &str = "PlayStore.enabled";
const _ABI_TYPE_KEY: &str = "abi.type";
const _AVD_DISPLAY_NAME_KEY: &str = "avd.ini.displayname";
const _AVD_ENCODING_KEY: &str = "avd.ini.encoding";
const _USERDATA_SIZE_KEY: &str = "disk.dataPartition.size";
const _FASTBOOT_CHOSEN_SNAPSHOT_FILE_KEY: &str = "fastboot.chosenSnapshotFile";
const _FASTBOOT_FORCE_CHOSEN_SNAPSHOT_BOOT_KEY: &str = "fastboot.forceChosenSnapshotBoot";
const _FASTBOOT_FORCE_COLD_BOOT_KEY: &str = "fastboot.forceColdBoot";
const _FASTBOOT_FORCE_FAST_BOOT_KEY: &str = "fastboot.forceFastBoot";
const _HW_ACCELEROMETER_KEY: &str = "hw.accelerometer";
const _HW_ARC_KEY: &str = "hw.arc";
const _HW_AUDIO_INPUT_KEY: &str = "hw.audioInput";
const _HW_BATTERY_KEY: &str = "hw.battery";
const _HW_CAMERA_BACK_KEY: &str = "hw.camera.back";
const _HW_CAMERA_FRONT_KEY: &str = "hw.camera.front";
const _HW_CPU_ARCH_KEY: &str = "hw.cpu.arch";
const _HW_CPU_NCORE_KEY: &str = "hw.cpu.ncore";
const _HW_DPAD_KEY: &str = "hw.dPad";
const _HW_DEVICE_HASH2_KEY: &str = "hw.device.hash2";
const _HW_DEVICE_MANUFACTURER_KEY: &str = "hw.device.manufacturer";
const _HW_DEVICE_NAME_KEY: &str = "hw.device.name";
const _HW_GPS_KEY: &str = "hw.gps";
const _HW_GPU_ENABLED_KEY: &str = "hw.gpu.enabled";
const _HW_GPU_MODE_KEY: &str = "hw.gpu.mode";
const _HW_INITIAL_ORIENTATION_KEY: &str = "hw.initialOrientation";
const _HW_KEYBOARD_KEY: &str = "hw.keyboard";
const _HW_LCD_DENSITY_KEY: &str = "hw.lcd.density";
const _HW_LCD_HEIGHT_KEY: &str = "hw.lcd.height";
const _HW_LCD_WIDTH_KEY: &str = "hw.lcd.width";
const _HW_MAIN_KEYS_KEY: &str = "hw.mainKeys";
const _HW_RAM_SIZE_KEY: &str = "hw.ramSize";
const _HW_SD_CARD_KEY: &str = "hw.sdCard";
const _HW_SENSORS_ORIENTATION_KEY: &str = "hw.sensors.orientation";
const _HW_SENSORS_PROXIMITY_KEY: &str = "hw.sensors.proximity";
const _HW_TRACK_BALL_KEY: &str = "hw.trackBall";
const _IMAGE_SYS_DIR_KEY: &str = "image.sysdir.1";
const _RUNTIME_NETWORK_LATENCY_KEY: &str = "runtime.network.latency";
const _RUNTIME_NETWORK_SPEED_KEY: &str = "runtime.network.speed";
const _SD_CARD_SIZE_KEY: &str = "sdcard.size";
const _SHOW_DEVICE_FRAME_KEY: &str = "showDeviceFrame";
const _SKIN_DYNAMIC_KEY: &str = "skin.dynamic";
const _SKIN_NAME_KEY: &str = "skin.name";
const _SKIN_PATH_KEY: &str = "skin.path";
const _SKIN_PATH_BACKUP_KEY: &str = "skin.path.backup";
const _TAG_DISPLAY_KEY: &str = "tag.display";
const _TAG_ID_KEY: &str = "tag.id";
const _VM_HEAP_SIZE_KEY: &str = "vm.heapSize";

pub struct AvdItem {
    pub avd_id: String,
    pub play_store_enabled: bool,
    pub abi_type: String,
    pub avd_display_name: String,
    pub avd_encoding: String,
    pub userdata_size: u64,
    pub fastboot_chosen_snapshot_file: String,
    pub fastboot_force_chosen_snapshot_boot: bool,
    pub fastboot_force_cold_boot: bool,
    pub fastboot_force_fast_boot: bool,
    pub hw_accelerometer: bool,
    pub hw_arc: bool,
    pub hw_audio_input: bool,
    pub hw_battery: bool,
    pub hw_camera_back: String,
    pub hw_camera_front: String,
    pub hw_cpu_arch: String,
    pub hw_cpu_ncore: u16,
    pub hw_dpad: bool,
    pub hw_device_hash2: String,
    pub hw_device_manufacturer: String,
    pub hw_device_name: String,
    pub hw_gps: bool,
    pub hw_gpu_enabled: bool,
    pub hw_gpu_mode: String,
    pub hw_initial_orientation: String,
    pub hw_keyboard: bool,
    pub hw_lcd_density: u16,
    pub hw_lcd_height: u16,
    pub hw_lcd_width: u16,
    pub hw_main_keys: bool,
    pub hw_ram_size: u32,
    pub hw_sd_card: bool,
    pub hw_sensors_orientation: bool,
    pub hw_sensors_proximity: bool,
    pub hw_track_ball: bool,
    pub image_sys_dir: String,
    pub runtime_network_latency: String,
    pub runtime_network_speed: String,
    pub sd_card_size: u64,
    pub show_device_frame: bool,
    pub skin_dynamic: bool,
    pub skin_name: String,
    pub skin_path: String,
    pub skin_path_backup: String,
    pub tag_display: String,
    pub tag_id: String,
    pub vm_heap_size: u16,
}

fn read_file_content(path: String) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    return contents;
}

fn parse_u16(value: &str) -> u16 {
    parse_u32(value) as u16
}

fn parse_u32(value: &str) -> u32 {
    parse_u64(value) as u32
}

fn parse_u64(value: &str) -> u64 {
    match value.parse::<u64>() {
        Ok(t) => return t,
        Err(_) => return parse_str_to_u64(value),
    }
}

fn parse_str_to_u64(value: &str) -> u64 {
    let split = value.split_at(value.len() - 1);

    let num: u64 = match split.0.parse::<u64>() {
        Ok(t) => t,
        Err(_) => 0,
    };
    
    match split.1 {
        "P" => 1125899906842624 * num,
        "T" => 1099511627776 * num,
        "G" => 1073741824 * num,
        "M" => 1048576 * num,
        "K" => 1024 * num,
        _ => return 0,
    }
}

fn parse_file(path: String) -> AvdItem {
    let contents = read_file_content(path);

    let mut avd_id: String = String::new();
    let mut play_store_enabled: bool = false;
    let mut abi_type: String = String::new();
    let mut avd_display_name: String = String::new();
    let mut avd_encoding: String = String::new();
    let mut userdata_size: u64 = 0;
    let mut fastboot_chosen_snapshot_file: String = String::new();
    let mut fastboot_force_chosen_snapshot_boot: bool = false;
    let mut fastboot_force_cold_boot: bool = false;
    let mut fastboot_force_fast_boot: bool = false;
    let mut hw_accelerometer: bool = false;
    let mut hw_arc: bool = false;
    let mut hw_audio_input: bool = false;
    let mut hw_battery: bool = false;
    let mut hw_camera_back: String = String::new();
    let mut hw_camera_front: String = String::new();
    let mut hw_cpu_arch: String = String::new();
    let mut hw_cpu_ncore: u16 = 0;
    let mut hw_dpad: bool = false;
    let mut hw_device_hash2: String = String::new();
    let mut hw_device_manufacturer: String = String::new();
    let mut hw_device_name: String = String::new();
    let mut hw_gps: bool = false;
    let mut hw_gpu_enabled: bool = false;
    let mut hw_gpu_mode: String = String::new();
    let mut hw_initial_orientation: String = String::new();
    let mut hw_keyboard: bool = false;
    let mut hw_lcd_density: u16 = 0;
    let mut hw_lcd_height: u16 = 0;
    let mut hw_lcd_width: u16 = 0;
    let mut hw_main_keys: bool = false;
    let mut hw_ram_size: u32 = 0;
    let mut hw_sd_card: bool = false;
    let mut hw_sensors_orientation: bool = false;
    let mut hw_sensors_proximity: bool = false;
    let mut hw_track_ball: bool = false;
    let mut image_sys_dir: String = String::new();
    let mut runtime_network_latency: String = String::new();
    let mut runtime_network_speed: String = String::new();
    let mut sd_card_size: u64 = 0;
    let mut show_device_frame: bool = false;
    let mut skin_dynamic: bool = false;
    let mut skin_name: String = String::new();
    let mut skin_path: String = String::new();
    let mut skin_path_backup: String = String::new();
    let mut tag_display: String = String::new();
    let mut tag_id: String = String::new();
    let mut vm_heap_size: u16 = 0;

    for line in contents.lines() {
        let pair: Vec<&str> = line.splitn(2, "=").collect();

        match pair[0].trim() {
            _AVD_ID_KEY => avd_id = pair[1].trim().to_string(),
            _PLAY_STORE_ENABLED_KEY => play_store_enabled = pair[1].trim().to_string() == "true",
            _ABI_TYPE_KEY => abi_type = pair[1].trim().to_string(),
            _AVD_DISPLAY_NAME_KEY => avd_display_name = pair[1].trim().to_string(),
            _AVD_ENCODING_KEY => avd_encoding = pair[1].trim().to_string(),
            _USERDATA_SIZE_KEY => userdata_size = parse_u64(pair[1].trim()),
            _FASTBOOT_CHOSEN_SNAPSHOT_FILE_KEY => fastboot_chosen_snapshot_file = pair[1].trim().to_string(),
            _FASTBOOT_FORCE_CHOSEN_SNAPSHOT_BOOT_KEY => fastboot_force_chosen_snapshot_boot = pair[1].trim() == "yes",
            _FASTBOOT_FORCE_COLD_BOOT_KEY => fastboot_force_cold_boot = pair[1].trim() == "yes",
            _FASTBOOT_FORCE_FAST_BOOT_KEY => fastboot_force_fast_boot = pair[1].trim() == "yes",
            _HW_ACCELEROMETER_KEY => hw_accelerometer = pair[1].trim() == "yes",
            _HW_ARC_KEY => hw_arc = pair[1].trim() == "yes",
            _HW_AUDIO_INPUT_KEY => hw_audio_input = pair[1].trim() == "yes",
            _HW_BATTERY_KEY => hw_battery = pair[1].trim() == "yes",
            _HW_CAMERA_BACK_KEY => hw_camera_back = pair[1].trim().to_string(),
            _HW_CAMERA_FRONT_KEY => hw_camera_front = pair[1].trim().to_string(),
            _HW_CPU_ARCH_KEY => hw_cpu_arch = pair[1].trim().to_string(),
            _HW_CPU_NCORE_KEY => hw_cpu_ncore = parse_u16(pair[1].trim()),
            _HW_DPAD_KEY => hw_dpad = pair[1].trim() == "yes",
            _HW_DEVICE_HASH2_KEY => hw_device_hash2 = pair[1].trim().to_string(),
            _HW_DEVICE_MANUFACTURER_KEY => hw_device_manufacturer = pair[1].trim().to_string(),
            _HW_DEVICE_NAME_KEY => hw_device_name = pair[1].trim().to_string(),
            _HW_GPS_KEY => hw_gps = pair[1].trim() == "yes",
            _HW_GPU_ENABLED_KEY => hw_gpu_enabled = pair[1].trim() == "yes",
            _HW_GPU_MODE_KEY => hw_gpu_mode = pair[1].trim().to_string(),
            _HW_INITIAL_ORIENTATION_KEY => hw_initial_orientation = pair[1].trim().to_string(),
            _HW_KEYBOARD_KEY => hw_keyboard = pair[1].trim() == "yes",
            _HW_MAIN_KEYS_KEY => hw_main_keys = pair[1].trim() == "yes",
            _HW_LCD_DENSITY_KEY => hw_lcd_density = parse_u16(pair[1].trim()),
            _HW_LCD_HEIGHT_KEY => hw_lcd_height = parse_u16(pair[1].trim()),
            _HW_LCD_WIDTH_KEY => hw_lcd_width = parse_u16(pair[1].trim()),
            _HW_MAIN_KEYS_KEY => hw_main_keys = pair[1].trim() == "yes",
            _HW_RAM_SIZE_KEY => hw_ram_size = parse_u32(pair[1].trim()),
            _HW_SD_CARD_KEY => hw_sd_card = pair[1].trim() == "yes",
            _HW_SENSORS_ORIENTATION_KEY => hw_sensors_orientation = pair[1].trim() == "yes",
            _HW_SENSORS_PROXIMITY_KEY => hw_sensors_proximity = pair[1].trim() == "yes",
            _HW_TRACK_BALL_KEY => hw_track_ball = pair[1].trim() == "yes",
            _IMAGE_SYS_DIR_KEY => image_sys_dir = pair[1].trim().to_string(),
            _RUNTIME_NETWORK_LATENCY_KEY => runtime_network_latency = pair[1].trim().to_string(),
            _RUNTIME_NETWORK_SPEED_KEY => runtime_network_speed = pair[1].trim().to_string(),
            _SD_CARD_SIZE_KEY => sd_card_size = parse_u64(pair[1].trim()),
            _SHOW_DEVICE_FRAME_KEY => show_device_frame = pair[1].trim() == "yes",
            _SKIN_DYNAMIC_KEY => skin_dynamic = pair[1].trim() == "yes",
            _SKIN_NAME_KEY => skin_name = pair[1].trim().to_string(),
            _SKIN_PATH_KEY => skin_path = pair[1].trim().to_string(),
            _SKIN_PATH_BACKUP_KEY => skin_path_backup = pair[1].trim().to_string(),
            _TAG_DISPLAY_KEY => tag_display = pair[1].trim().to_string(),
            _TAG_ID_KEY => tag_id = pair[1].trim().to_string(),
            _VM_HEAP_SIZE_KEY => vm_heap_size = parse_u16(pair[1].trim()),
            _ => {}
        }
    }


    AvdItem {
        avd_id: avd_id,
        play_store_enabled: play_store_enabled,
        abi_type: abi_type, 
        avd_display_name: avd_display_name, 
        avd_encoding: avd_encoding, 
        userdata_size: userdata_size, 
        fastboot_chosen_snapshot_file: fastboot_chosen_snapshot_file, 
        fastboot_force_chosen_snapshot_boot: fastboot_force_chosen_snapshot_boot, 
        fastboot_force_cold_boot: fastboot_force_cold_boot, 
        fastboot_force_fast_boot: fastboot_force_fast_boot, 
        hw_accelerometer: hw_accelerometer, 
        hw_arc: hw_arc, 
        hw_audio_input: hw_audio_input, 
        hw_battery: hw_battery, 
        hw_camera_back: hw_camera_back, 
        hw_camera_front: hw_camera_front, 
        hw_cpu_arch: hw_cpu_arch, 
        hw_cpu_ncore: hw_cpu_ncore, 
        hw_dpad: hw_dpad, 
        hw_device_hash2: hw_device_hash2, 
        hw_device_manufacturer: hw_device_manufacturer, 
        hw_device_name: hw_device_name, 
        hw_gps: hw_gps, 
        hw_gpu_enabled: hw_gpu_enabled, 
        hw_gpu_mode: hw_gpu_mode, 
        hw_initial_orientation: hw_initial_orientation, 
        hw_keyboard: hw_keyboard, 
        hw_lcd_density: hw_lcd_density, 
        hw_lcd_height: hw_lcd_height, 
        hw_lcd_width: hw_lcd_width, 
        hw_main_keys: hw_main_keys, 
        hw_ram_size: hw_ram_size, 
        hw_sd_card: hw_sd_card, 
        hw_sensors_orientation: hw_sensors_orientation, 
        hw_sensors_proximity: hw_sensors_proximity, 
        hw_track_ball: hw_track_ball, 
        image_sys_dir: image_sys_dir, 
        runtime_network_latency: runtime_network_latency, 
        runtime_network_speed: runtime_network_speed, 
        sd_card_size: sd_card_size, 
        show_device_frame: show_device_frame, 
        skin_dynamic: skin_dynamic, 
        skin_name: skin_name, 
        skin_path: skin_path, 
        skin_path_backup: skin_path_backup, 
        tag_display: tag_display, 
        tag_id: tag_id, 
        vm_heap_size: vm_heap_size,
    }
}


pub fn list_avds() -> LinkedList<AvdItem> {
    let sdk_home: String = env::var("ANDROID_SDK_HOME").expect("Please set ANDROID_SDK_HOME variable!");
    let mut out: LinkedList<AvdItem> = LinkedList::new();

    let android_dir: &str = ".android/avd";
    let mut avd_dir: String = sdk_home + "/" + android_dir;
    if ! Path::new(&avd_dir).exists() {
        let home: String = env::var("HOME").expect("Please set HOME variable!");
        let avd_alt_dir: String = home + "/" + android_dir;

        if Path::new(&avd_alt_dir).exists() {
           avd_dir = avd_alt_dir;
        } else {
            return out;
        }
    }

    let items: ReadDir = fs::read_dir(avd_dir).unwrap();

    for item in items {
        let mut path: String = item.unwrap().path().display().to_string();

        if ! path.ends_with(".avd") {
            continue;
        }

        path += "/config.ini";

        let section: AvdItem = parse_file(path);
        out.push_back(section);
    }

    out
}
