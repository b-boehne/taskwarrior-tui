use serde::{Deserialize, Serialize};

use crate::util::{Key, Events, Event};

pub struct KeyConfig {
    pub exit: Key,
    pub exit_popup: Key,
    pub enter: Key,
    pub home: Key,
    pub end: Key,
    pub move_up: Key,
    pub move_down: Key,
    pub move_up_page: Key,
    pub move_down_page: Key,
    pub move_top_of_list: Key,
    pub move_bottom_of_list: Key,
    pub edit_task: Key,
    pub delete_task: Key,
    pub add_task: Key,
    pub annotate_task: Key,
    pub modify_task: Key,
    pub filter_tasks: Key,
    pub next_view: Key,
    pub toggle_task_info: Key,
    pub previous_view: Key,
    pub shell_command: Key,
    pub context_switcher_menu: Key,
    pub help_menu: Key,
}

impl Default for KeyConfig {
    fn default() -> Self {

        let exit = Key::Char('q');
        let exit_popup = Key::Esc;
        let enter = Key::Char('\n');
        let home = Key::Home;
        let end = Key::End;
        let move_down = Key::Char('j');
        let move_up = Key::Char('k');
        let move_down_page = Key::Char('J');
        let move_up_page = Key::Char('K');
        let move_top_of_list = Key::Char('g');
        let move_bottom_of_list = Key::Char('G');
        let edit_task = Key::Char('e');
        let delete_task = Key::Char('d');
        let add_task = Key::Char('a');
        let annotate_task = Key::Char('A');
        let modify_task = Key::Char('m');
        let filter_tasks = Key::Char('/');
        let previous_view = Key::Char('[');
        let next_view = Key::Char(']');
        let toggle_task_info = Key::Char('z');
        let shell_command = Key::Char('!');
        let context_switcher_menu = Key::Char('c');
        let help_menu = Key::Char('?');

        Self {
            exit,
            exit_popup,
            enter,
            home,
            end,
            move_up,
            move_down,
            move_up_page,
            move_down_page,
            move_top_of_list,
            move_bottom_of_list,
            edit_task,
            delete_task,
            add_task,
            annotate_task,
            modify_task,
            filter_tasks,
            next_view,
            toggle_task_info,
            previous_view,
            shell_command,
            context_switcher_menu,
            help_menu,
        }

    }
}
