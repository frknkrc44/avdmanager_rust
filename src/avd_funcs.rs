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

use slint::{StandardListViewItem, ModelRc, VecModel};

use crate::avd_item::AvdItem;
use crate::parser_funcs::*;
use std::collections::LinkedList;
use std::env::var;
use std::fs::read_dir;
use std::path::Path;

pub fn list_avds() -> LinkedList<AvdItem> {
    let sdk_home: String = var("ANDROID_SDK_HOME").expect("Please set ANDROID_SDK_HOME variable!");
    let mut out: LinkedList<AvdItem> = LinkedList::new();

    let android_dir: &str = ".android/avd";
    let mut avd_dir: String = sdk_home + "/" + android_dir;

    if !Path::new(&avd_dir).exists() {
        let home: String = var("HOME").expect("Please set HOME variable!");
        let avd_alt_dir: String = home + "/" + android_dir;

        if Path::new(&avd_alt_dir).exists() {
            avd_dir = avd_alt_dir;
        } else {
            return out;
        }
    }

    for file in read_dir(avd_dir).unwrap() {
        let mut path: String = file.unwrap().path().display().to_string();

        if !path.ends_with(".avd") {
            continue;
        }

        path += "/config.ini";
        out.push_back(parse_ini_to_avd(path));
    }

    out
}

pub fn get_avds_as_slint_model() -> ModelRc<ModelRc<StandardListViewItem>> {
    let vecs: Vec<ModelRc<StandardListViewItem>> = list_avds().iter().map(|s| {
        let item = vec!(
            get_list_view_item(&s.avd_display_name),
            get_list_view_item(&s.avd_sdk_level.to_string()),
            get_list_view_item(&s.tag_display),
            get_list_view_item(&s.abi_type),
        );

        ModelRc::new(VecModel::from(item))
    }).collect();

    ModelRc::new(VecModel::from(vecs))
}
