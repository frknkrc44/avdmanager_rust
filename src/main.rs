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

#![allow(unused_assignments)]

use std::{process::{Command, Output, ExitStatus},  rc::Rc, cell::RefCell, borrow::BorrowMut};

use avd_funcs::AvdList;
use slint::{WindowSize, LogicalSize, Weak};
slint::include_modules!();

mod avd_funcs;
mod avd_item;
mod r#const;
mod parser_funcs;

fn on_row_changed(row: i32, ui_handle: &Weak<AppWindow>, avds: &Rc<RefCell<AvdList>>) {
    let ui = ui_handle.unwrap();

    if row < 0 {
        ui.set_edit_btn_enabled(false);
        ui.set_delete_btn_enabled(false);
        ui.set_repair_btn_enabled(false);
        ui.set_details_btn_enabled(false);
        ui.set_start_btn_enabled(false);
        return
    }

    let binding = avds.borrow();
    let info = binding.iter().nth(row as usize).unwrap();
    let mut cmd = Command::new("pgrep");

    let cmd_output = match cmd.arg("-a").arg("qemu-system-").output() {
        Ok(a) => a,
        Err(_) => Output {
            status: ExitStatus::default(),
            stdout: Vec::new(),
            stderr: Vec::new(),
        },
    };

    let out = String::from_utf8(cmd_output.stdout.to_vec()).unwrap();
    let choose1 = "-avd ".to_owned() + &info.avd_id;
    let choose2 = "@".to_owned() + &info.avd_id;
    let out = out.split('\n').find(|e| e.contains(&choose1) || e.contains(&choose2)).unwrap_or("").trim();

    ui.set_details_btn_enabled(true);
    ui.set_edit_btn_enabled(out.is_empty());
    ui.set_delete_btn_enabled(out.is_empty());
    ui.set_repair_btn_enabled(out.is_empty());
    ui.set_start_btn_enabled(out.is_empty());
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.window().set_size(WindowSize::Logical(LogicalSize::new(640_f32, 480_f32)));

    let avds: avd_funcs::AvdList = avd_funcs::list_avds();
    let mut avds = Rc::new(RefCell::new(avds));
    let avds_read_clone = Rc::clone(&avds);
    ui.set_avdlist(avd_funcs::convert_avd_list_to_slint_model(&avds_read_clone.borrow()));

    let ui_handle = ui.as_weak();
    let ui_handle_clone = Rc::new(RefCell::new(ui_handle.clone()));
    ui.on_table_current_row_changed(move |row: i32| {
        on_row_changed(row, &ui_handle, &avds_read_clone)
    });

    ui.on_edit_btn_press(move || {
        // dummy
    });

    ui.on_delete_btn_press(move || {
        // dummy
    });

    ui.on_repair_btn_press(move || {
        // dummy
    });

    ui.on_details_btn_press(move || {
        // dummy
    });

    ui.on_start_btn_press(move || {
        // dummy
    });

    ui.on_refresh_btn_press(move || {
        let ui = ui_handle_clone.borrow().unwrap();
        let new_avds: avd_funcs::AvdList = avd_funcs::list_avds();
        *avds.borrow_mut() = Rc::new(RefCell::new(new_avds.clone()));
        ui.set_avdlist(avd_funcs::convert_avd_list_to_slint_model(&new_avds));
    });

    ui.run()
}
