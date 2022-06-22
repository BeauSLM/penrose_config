#[macro_use]
extern crate penrose;

use penrose::{
    core::{
        // bindings::KeyEventHandler,
        config::Config,
        helpers::index_selectors,
        // manager::WindowManager,
        layout::{left_stack, monocle, Layout, LayoutConf}
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More, Selector
};

// imports from lib.rs
use penrose_config::*;

use simplelog::{LevelFilter, SimpleLogger};

fn main() -> penrose::Result<()> {
    // Initialise the logger (use LevelFilter::Debug to enable debug logging)
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };
    
    // run startup script
    // QUESTION: should this be a hook? I didn't see the point
    spawn!("/home/beaum/.config/startup.sh")?;

    let config = Config::default()
        .builder()
        .workspaces(["1", "2", "3", "4", "5", "6", "7", "8", "9"])
        .layouts(vec![
            Layout::new(TILE_SYMBOL, LayoutConf::default(), left_stack, N_MAIN, RATIO),
            Layout::new(MONO_SYMBOL, LayoutConf { follow_focus: true, ..Default::default() }, monocle, N_MAIN, RATIO),
            Layout::floating(FLOAT_SYMBOL),
        ])
        .show_bar(true)
        .bar_height(40)
        .build()
        .expect("built config successfully");
    
    let key_bindings = gen_keybindings! {
        "M-semicolon" => run_external!(LAUNCHER);
        "M-Return" => run_external!(TERMINAL);
        
        // Exit Penrose (important to remember this one!)
        "M-S-Q" => run_internal!(exit);

        // client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-q" => run_internal!(kill_client);
        
        // screen cycle
        "M-h" => run_internal!(cycle_screen, Backward);
        "M-l" => run_internal!(cycle_screen, Forward);

        // workspace management
        "M-Tab" => run_internal!(toggle_workspace);
        "M-period" => run_internal!(cycle_workspace, Forward);
        "M-comma" => run_internal!(cycle_workspace, Backward);

        // Layout management
        // TODO: pick specific layout keybinds
        "M-grave" => run_internal!(cycle_layout, Forward);
        "M-S-grave" => run_internal!(cycle_layout, Backward);
        "M-i" => run_internal!(update_max_main, More);
        "M-d" => run_internal!(update_max_main, Less);
        "M-S-h" => run_internal!(update_main_ratio, More);
        "M-S-l" => run_internal!(update_main_ratio, Less);

        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
        };
    };

    let mut wm = new_xcb_backed_window_manager(config, vec![], logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map!{})
}
