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

mod avd_item;
mod r#const;
mod avd_funcs;
mod parser_funcs;

fn main() {
    let avds: LinkedList<avd_item::AvdItem> = avd_funcs::list_avds();

    for avd in avds {
        println!("{}, {}", avd.avd_display_name, avd.userdata_size);
        let parsed: String = parser_funcs::parse_avd_to_ini(avd);
        println!("{}", parsed);
    }
}
