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

 #[derive(Clone)]
pub struct AvdItem {
    pub avd_id: String,
    pub play_store_enabled: bool,
    pub abi_type: String,
    pub avd_display_name: String,
    pub avd_encoding: String,
    pub avd_sdk_level: u16,
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
