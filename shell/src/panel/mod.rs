use std::rc::Rc;
use serde::{Serialize, Deserialize};
use gtk;
use gtk::prelude::*;
use relm::{Relm, Update, Widget, Component, ContainerWidget};
use relm_derive::Msg;
use protos::gtkclient;
use crate::launcher;

use self::Msg::*;

mod launcher_button;
mod quick_launch;
mod clock;

#[derive(Debug, Serialize, Deserialize)]
pub struct SeparatorConfig {
    stretch: bool,
    padding: u32,
}

impl Default for SeparatorConfig {
    fn default() -> SeparatorConfig {
        SeparatorConfig {
            stretch: false,
            padding: 5,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WidgetConfig {
    Separator(SeparatorConfig),
    LauncherButton(launcher_button::LauncherButtonConfig),
    QuickLaunch(quick_launch::LaunchConfig),
    Clock(clock::ClockConfig),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PanelConfig {
    widgets: Vec<WidgetConfig>
}

impl Default for PanelConfig {
    fn default() -> PanelConfig {
        PanelConfig {
            widgets: vec![
                WidgetConfig::LauncherButton(Default::default()),
                WidgetConfig::QuickLaunch(Default::default()),
                WidgetConfig::Separator(SeparatorConfig {
                    stretch: true,
                    padding: 5,
                }),
                WidgetConfig::Separator(Default::default()),
                WidgetConfig::Clock(Default::default()),
            ]
        }
    }
}

enum WidgetComponent {
    Separator(gtk::Separator),
    LauncherButton(Component<launcher_button::LauncherButton>),
    QuickLaunch(Component<quick_launch::Launch>),
    Clock(Component<clock::Clock>),
}

pub struct Model {
    layer_shell: gtkclient::LayerShellApi,
    dank_private: gtkclient::DankShellApi,
    launcher: Rc<Component<launcher::Launcher>>,
    config: PanelConfig,
}

#[derive(Msg)]
pub enum Msg {
    Reconfigure(PanelConfig),
}

pub struct Panel {
    model: Model,
    window: gtk::Window,
    hbox: gtk::Box,
    components: Vec<WidgetComponent>,
}

impl Update for Panel {
    type Model = Model;
    type ModelParam = (gtkclient::LayerShellApi, gtkclient::DankShellApi, Rc<Component<launcher::Launcher>>);
    type Msg = Msg;

    fn model(_relm: &Relm<Self>, (layer_shell, dank_private, launcher):
             (gtkclient::LayerShellApi, gtkclient::DankShellApi, Rc<Component<launcher::Launcher>>)) -> Model {
        Model {
            layer_shell,
            dank_private,
            launcher,
            config: Default::default(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Reconfigure(new_config) => {
                // Would be cool to diff and modify the config on existing components?
                // But for now just delete and recreate.
                for component in self.components.drain(0..) {
                    use self::WidgetComponent::*;
                    match component {
                        Separator(c) => self.hbox.remove(&c),
                        LauncherButton(c) => self.hbox.remove_widget(c),
                        QuickLaunch(c) => self.hbox.remove_widget(c),
                        Clock(c) => self.hbox.remove_widget(c),
                    };
                }
                self.model.config = new_config;
                setup_widgets(&mut self.components, &self.hbox, &self.model);
            }
        }
    }
}

impl Widget for Panel {
    type Root = gtk::Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(_relm: &Relm<Self>, mut model: Model) -> Self {
        let mut window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_title("Panel");
        window.set_default_size(640, 32);
        window.set_decorated(false);
        use protos::gtkclient::lsr::{Anchor, RequestsTrait};
        let layer_surface = gtkclient::get_layer_surface(&mut model.layer_shell, &mut window, gtkclient::lsh::Layer::Top);
        layer_surface.set_anchor(Anchor::Bottom | Anchor::Left | Anchor::Right);
        layer_surface.set_size(640, 32);

        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        window.add(&hbox);

        let mut components = Vec::new();
        setup_widgets(&mut components, &hbox, &model);

        window.show_all();
        Panel {
            model,
            window,
            hbox,
            components,
        }
    }
}

fn setup_widgets(components: &mut Vec<WidgetComponent>, hbox: &gtk::Box, model: &Model) {
    for widget in &model.config.widgets {
        use self::WidgetConfig::*;
        let component = match widget {
            Separator(SeparatorConfig { stretch, padding }) => {
                use gtk::BoxExt;
                let sep = gtk::Separator::new(gtk::Orientation::Vertical);
                hbox.pack_start(&sep, *stretch, true, *padding);
                if *stretch {
                    sep.set_opacity(0.0);
                }
                sep.show();
                WidgetComponent::Separator(sep)
            },
            LauncherButton(conf) => WidgetComponent::LauncherButton(hbox.add_widget::<launcher_button::LauncherButton>((conf.clone(), model.launcher.clone()))),
            QuickLaunch(conf) => WidgetComponent::QuickLaunch(hbox.add_widget::<quick_launch::Launch>((conf.clone(), model.dank_private.clone()))),
            Clock(conf) => WidgetComponent::Clock(hbox.add_widget::<clock::Clock>(conf.clone())),
        };
        components.push(component);
    }
}
