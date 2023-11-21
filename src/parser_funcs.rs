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

use crate::avd_item::AvdItem;
use crate::r#const::*;
use std::fs::File;
use std::io::Read;

pub fn parse_ini_to_avd(path: String) -> AvdItem {
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

pub fn parse_avd_to_ini(item: AvdItem) -> String {
    let mut out: String = String::new();
    out += &get_parsed_value(_AVD_ID_KEY, &item.avd_id);
    out += &parse_bool_to_ini(_PLAY_STORE_ENABLED_KEY, item.play_store_enabled, false);
    out += &get_parsed_value(_ABI_TYPE_KEY, &item.abi_type);
    out += &get_parsed_value(_AVD_DISPLAY_NAME_KEY, &item.avd_display_name);
    out += &get_parsed_value(_AVD_ENCODING_KEY, &item.avd_encoding);
    out += &parse_u64_to_ini(_USERDATA_SIZE_KEY, item.userdata_size);
    out += &get_parsed_value(_FASTBOOT_CHOSEN_SNAPSHOT_FILE_KEY, &item.fastboot_chosen_snapshot_file);
    out += &parse_bool_to_ini(_FASTBOOT_FORCE_CHOSEN_SNAPSHOT_BOOT_KEY, item.fastboot_force_chosen_snapshot_boot, true);
    out += &parse_bool_to_ini(_FASTBOOT_FORCE_COLD_BOOT_KEY, item.fastboot_force_cold_boot, true);
    out += &parse_bool_to_ini(_FASTBOOT_FORCE_FAST_BOOT_KEY, item.fastboot_force_fast_boot, true);
    out += &parse_bool_to_ini(_HW_ACCELEROMETER_KEY, item.hw_accelerometer, true);
    out += &parse_bool_to_ini(_HW_ARC_KEY, item.hw_arc, true);
    out += &parse_bool_to_ini(_HW_AUDIO_INPUT_KEY, item.hw_audio_input, true);
    out += &parse_bool_to_ini(_HW_BATTERY_KEY, item.hw_battery, true);
    out += &get_parsed_value(_HW_CAMERA_BACK_KEY, &item.hw_camera_back);
    out += &get_parsed_value(_HW_CAMERA_FRONT_KEY, &item.hw_camera_front);
    out += &get_parsed_value(_HW_CPU_ARCH_KEY, &item.hw_cpu_arch);
    out += &parse_u16_to_ini(_HW_CPU_NCORE_KEY, item.hw_cpu_ncore);
    out += &parse_bool_to_ini(_HW_DPAD_KEY, item.hw_dpad, true);
    out += &get_parsed_value(_HW_DEVICE_HASH2_KEY, &item.hw_device_hash2);
    out += &get_parsed_value(_HW_DEVICE_MANUFACTURER_KEY, &item.hw_device_manufacturer);
    out += &get_parsed_value(_HW_DEVICE_NAME_KEY, &item.hw_device_name);
    out += &parse_bool_to_ini(_HW_GPS_KEY, item.hw_gps, true);
    out += &parse_bool_to_ini(_HW_GPU_ENABLED_KEY, item.hw_gpu_enabled, true);
    out += &get_parsed_value(_HW_GPU_MODE_KEY, &item.hw_gpu_mode);
    out += &get_parsed_value(_HW_INITIAL_ORIENTATION_KEY, &item.hw_initial_orientation);
    out += &parse_bool_to_ini(_HW_KEYBOARD_KEY, item.hw_keyboard, true);
    out += &parse_u16_to_ini(_HW_LCD_DENSITY_KEY, item.hw_lcd_density);
    out += &parse_u16_to_ini(_HW_LCD_HEIGHT_KEY, item.hw_lcd_height);
    out += &parse_u16_to_ini(_HW_LCD_WIDTH_KEY, item.hw_lcd_width);
    out += &parse_bool_to_ini(_HW_MAIN_KEYS_KEY, item.hw_main_keys, true);
    out += &parse_u32_to_ini(_HW_RAM_SIZE_KEY, item.hw_ram_size);
    out += &parse_bool_to_ini(_HW_SD_CARD_KEY, item.hw_sd_card, true);
    out += &parse_bool_to_ini(_HW_SENSORS_ORIENTATION_KEY, item.hw_sensors_orientation, true);
    out += &parse_bool_to_ini(_HW_SENSORS_PROXIMITY_KEY, item.hw_sensors_proximity, true);
    out += &parse_bool_to_ini(_HW_TRACK_BALL_KEY, item.hw_track_ball, true);
    out += &get_parsed_value(_IMAGE_SYS_DIR_KEY, &item.image_sys_dir);
    out += &get_parsed_value(_RUNTIME_NETWORK_LATENCY_KEY, &item.runtime_network_latency);
    out += &get_parsed_value(_RUNTIME_NETWORK_SPEED_KEY, &item.runtime_network_speed);
    out += &parse_u64_to_ini(_SD_CARD_SIZE_KEY, item.sd_card_size);
    out += &parse_bool_to_ini(_SHOW_DEVICE_FRAME_KEY, item.show_device_frame, true);
    out += &parse_bool_to_ini(_SKIN_DYNAMIC_KEY, item.skin_dynamic, true);
    out += &get_parsed_value(_SKIN_NAME_KEY, &item.skin_name);
    out += &get_parsed_value(_SKIN_PATH_KEY, &item.skin_path);
    out += &get_parsed_value(_SKIN_PATH_BACKUP_KEY, &item.skin_path_backup);
    out += &get_parsed_value(_TAG_DISPLAY_KEY, &item.tag_display);
    out += &get_parsed_value(_TAG_ID_KEY, &item.tag_id);
    out += &parse_u16_to_ini(_VM_HEAP_SIZE_KEY, item.vm_heap_size);

    out
}

fn read_file_content(path: String) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    contents
}

fn parse_u16(value: &str) -> u16 {
    parse_u32(value) as u16
}

fn parse_u32(value: &str) -> u32 {
    parse_u64(value) as u32
}

fn parse_u64(value: &str) -> u64 {
    match value.parse::<u64>() {
        Ok(t) => t,
        Err(_) => parse_str_to_u64(value),
    }
}

fn parse_str_to_u64(value: &str) -> u64 {
    let split = value.split_at(value.len() - 1);

    let num: u64 = match split.0.parse::<u64>() {
        Ok(t) => t,
        Err(_) => 0,
    };

    match split.1 {
        "P" => PIB * num,
        "T" => TIB * num,
        "G" => GIB * num,
        "M" => MIB * num,
        "K" => KIB * num,
        _ => 0,
    }
}

fn parse_u64_to_str<'a>(value: u64) -> String {
    if value <= KIB {
        return value.to_string()
    }

    if value <= MIB {
        return (value / KIB).to_string() + "K"
    }

    if value <= GIB {
        return (value / MIB).to_string() + "M"
    }

    if value <= TIB {
        return (value / GIB).to_string() + "G"
    }

    if value <= PIB {
        return (value / TIB).to_string() + "T"
    }

    (value / PIB).to_string() + "P"
}

fn parse_u16_to_ini(key: &str, value: u16) -> String {
    parse_u32_to_ini(key, value as u32)
}

fn parse_u32_to_ini(key: &str, value: u32) -> String {
    parse_u64_to_ini(key, value as u64)
}

fn parse_u64_to_ini(key: &str, value: u64) -> String {
    get_parsed_value(key, &parse_u64_to_str(value))
}

fn parse_bool_to_ini(key: &str, value: bool, yes_no: bool) -> String {
    let parsed_bool: String = match yes_no {
        true => String::from(
            match value {
                true => "yes",
                _ => "no",
            }
        ),
        _ => String::from(
            match value {
                true => "true",
                _ => "false",
            }
        ),
    };

    get_parsed_value(key, &parsed_bool)
}

fn get_parsed_value<'a>(key: &'a str, value: &'a str) -> String {
    key.to_owned() + "=" + value + "\n"
}
