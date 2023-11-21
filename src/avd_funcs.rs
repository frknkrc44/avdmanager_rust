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
use crate::parser_funcs::parse_ini_to_avd;
use std::collections::LinkedList;
use std::env;
use std::fs;
use std::path::Path;

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

    for file in fs::read_dir(avd_dir).unwrap() {
        let mut path: String = file.unwrap().path().display().to_string();

        if ! path.ends_with(".avd") {
            continue;
        }

        path += "/config.ini";
        out.push_back(parse_ini_to_avd(path));
    }

    out
}
