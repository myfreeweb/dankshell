#![feature(nll)]

extern crate libc;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate loginw;
#[macro_use]
extern crate weston_rs;
#[macro_use]
extern crate lazy_static;
extern crate mut_static;
extern crate protos;

use std::{env, process};
use mut_static::MutStatic;
use weston_rs::*;
use loginw::priority;

mod backend;
mod ctx;
mod moove;
mod resize;
mod desktop;
mod focus;
mod layer_shell;

use ctx::SurfaceContext;

lazy_static! {
    static ref COMPOSITOR: MutStatic<Compositor> = MutStatic::new();
    static ref DESKTOP: MutStatic<Desktop<SurfaceContext>> = MutStatic::new();
}

weston_logger!{fn wlog(msg: &str) {
    info!(target: "weston", "{}", msg);
}}

fn main() {
    pretty_env_logger::init();
    weston_rs::log_set_handler(wlog, wlog);

    let (mut display, mut event_loop) = Display::new();
    let mut compositor = Compositor::new(&display, &mut event_loop);

    compositor.set_xkb_rule_names(None); // defaults to environment variables

    // Backend/head/output setup
    let be = backend::start_backend(&mut compositor, &mut event_loop);
    compositor.add_heads_changed_listener(backend::heads_changed_listener(be));
    compositor.flush_heads_changed();

    // Background color
    let mut bg_layer = Layer::new(&compositor);
    bg_layer.set_position(POSITION_BACKGROUND);
    let mut bg_surf = Surface::new(&compositor);
    bg_surf.set_size(8096, 8096);
    bg_surf.set_color(0.1, 0.3, 0.6, 1.0);
    let mut bg_view = View::new(&bg_surf);
    bg_layer.view_list_entry_insert(&mut bg_view);

    // Our data for libweston-desktop stuff
    let desktop_impl = Box::new(desktop::DesktopImpl::new(&compositor));

    // The libweston-desktop object
    // NOTE: Important to keep around (do not do 'let _')
    let desktop = Desktop::new(unsafe { CompositorRef::from_ptr(compositor.as_ptr()) }, desktop_impl);

    // Left click to focus window
    let _ = compositor.add_button_binding(ev::BTN_LEFT, KeyboardModifier::empty(), &|p, _, _| focus::click_activate(p));
    // Right click to focus window
    let _ = compositor.add_button_binding(ev::BTN_RIGHT, KeyboardModifier::empty(), &|p, _, _| focus::click_activate(p));

    focus::keyboard_focus_listener().signal_add(
        compositor.first_seat().expect("first_seat")
        .keyboard().expect("first_seat keyboard")
        .focus_signal());

    // Ctrl+Enter to spawn a terminal
    compositor.add_key_binding(ev::KEY_ENTER, KeyboardModifier::CTRL, &|_, _, _| {
        use std::os::unix::process::CommandExt;
        let _ = process::Command::new("weston-terminal").before_exec(|| {
            // loginw sets realtime priority for the compositor
            // see https://blog.martin-graesslin.com/blog/2017/09/kwinwayland-goes-real-time/ for reasons
            // we obviously don't want it in user applications :D
            priority::make_normal();
            Ok(())
        }).spawn().expect("spawn");
    });

    // Set environment for spawned processes (namely, the terminal above)
    env::remove_var("DISPLAY");
    let sock_name = display.add_socket_auto().expect("add_socket_auto");
    env::set_var("WAYLAND_DISPLAY", sock_name);

    // Setup layer-shell
    layer_shell::create_layers(&compositor);
    layer_shell::register_layer_shell(&mut display, event_loop.token());

    // Go!
    compositor.wake();
    COMPOSITOR.set(compositor).expect("compositor MutStatic set");
    DESKTOP.set(desktop).expect("desktop MutStatic set");
    let _ = event_loop.run();
}
