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
use std::process::{Command, Output, ExitStatus};

pub type AvdList = LinkedList<AvdItem>;
pub type AvdRows = ModelRc<ModelRc<StandardListViewItem>>;
pub type RunningList = LinkedList<(u32, String)>;

pub fn list_avds() -> AvdList {
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

pub fn convert_avd_list_to_slint_model(items: &AvdList) -> AvdRows {
    let vecs: Vec<ModelRc<StandardListViewItem>> = items.iter().map(|s: &AvdItem| {
        let item: Vec<StandardListViewItem> = vec!(
            get_list_view_item(&s.avd_display_name),
            get_list_view_item(&s.avd_sdk_level.to_string()),
            get_list_view_item(&s.tag_display),
            get_list_view_item(&s.abi_type),
        );

        ModelRc::new(VecModel::from(item))
    }).collect();

    ModelRc::new(VecModel::from(vecs))
}

pub fn get_default_output() -> Output {
    Output {
        status: ExitStatus::default(),
        stdout: Vec::new(),
        stderr: Vec::new(),
    }
}

pub fn filter_running_avds(items: &AvdList) -> RunningList {
    let mut cmd = Command::new("pgrep");
    let cmd_output = match cmd.arg("-a").arg("qemu-system-").output() {
        Ok(a) => a,
        Err(_) => get_default_output(),
    };

    let stdout = String::from_utf8(cmd_output.stdout.to_vec()).unwrap();
    let mut out_list: RunningList = LinkedList::new();

    for item in items.iter() {
        let id = &item.avd_id;
        let choose1 = "-avd ".to_owned() + &id;
        let choose2 = "@".to_owned() + &id;
        let filter = stdout.split('\n').find(|e| e.contains(&choose1) || e.contains(&choose2)).unwrap_or("").trim();
        if !filter.is_empty() {
            let pid = filter.split(' ').nth(0).unwrap_or("").parse::<u32>().unwrap_or(0);
            out_list.push_back((pid, id.to_string()));
        }
    }

    out_list
}

pub fn is_avd_running(items: &AvdList, avd_id: &str) -> u32 {
    let running = filter_running_avds(items);
    let contains: Vec<&(u32, String)> = running.iter().filter(|e| e.1 == avd_id).collect();
    
    if contains.is_empty() {
        0
    } else {
        contains.first().unwrap().0
    }
}
