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

use crate::avd_item::AvdItem;
use crate::avd_item_keys::*;

pub fn parse_u16(value: &str) -> u16 {
    parse_u32(value) as u16
}

pub fn parse_u32(value: &str) -> u32 {
    parse_u64(value) as u32
}

pub fn parse_u64(value: &str) -> u64 {
    match value.parse::<u64>() {
        Ok(t) => return t,
        Err(_) => return parse_str_to_u64(value),
    }
}

pub fn parse_str_to_u64(value: &str) -> u64 {
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
        _ => 0,
    }
}


fn parse_u64_to_str<'a>(value: u64) -> String {
    if value <= 1024 {
        return value.to_string()
    }

    if value <= 1048576 {
        return (value / 1024).to_string() + "K"
    }

    if value <= 1073741824 {
        return (value / 1048576).to_string() + "M"
    }

    if value <= 1099511627776 {
        return (value / 1073741824).to_string() + "G"
    }

    if value <= 1125899906842624 {
        return (value / 1099511627776).to_string() + "T"
    }

    return (value / 1125899906842624).to_string() + "P"
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
