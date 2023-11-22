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

use std::collections::LinkedList;
slint::include_modules!();

mod avd_funcs;
mod avd_item;
mod r#const;
mod parser_funcs;

fn main() -> Result<(), slint::PlatformError> {
    let avds: LinkedList<avd_item::AvdItem> = avd_funcs::list_avds();
    let mut avd_names: String = String::new();
    for avd in avds {
        //println!("{}, {}", avd.avd_display_name, avd.userdata_size);
        //let parsed: String = parser_funcs::parse_avd_to_ini(avd);
        avd_names = avd.avd_display_name + "\n" + &avd_names;
        //println!("{}", parsed);
    }

    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_avd_list(move || {
        let ui = ui_handle.unwrap();
        ui.set_avdlist(ui.get_avdlist() + &avd_names);
    });

    ui.run()
}
