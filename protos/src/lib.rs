#[macro_use]
extern crate wayland_sys;
extern crate wayland_protocols;
extern crate wayland_commons;
#[cfg(feature = "client")]
extern crate wayland_client;
#[cfg(feature = "server")]
extern crate wayland_server;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate serde_derive;
extern crate serde_cbor;
extern crate serde;
#[cfg(feature = "gtkclient")]
extern crate libc;
#[cfg(feature = "gtkclient")]
#[macro_use]
extern crate log;
#[cfg(feature = "gtkclient")]
extern crate gtk;
#[cfg(feature = "gtkclient")]
extern crate gdk;
#[cfg(feature = "gtkclient")]
extern crate gdk_sys;
#[cfg(feature = "gtkclient")]
extern crate glib;
#[cfg(feature = "gtkclient")]
extern crate fragile;

use serde::{Serialize, de::DeserializeOwned};

#[cfg_attr(feature = "cargo-clippy", allow(clippy))]
#[allow(dead_code,non_camel_case_types,unused_unsafe,unused_variables)]
#[allow(non_upper_case_globals,non_snake_case,unused_imports,missing_docs)]
pub mod layer_shell {
    #[cfg(feature = "client")]
    pub use self::generated::client::c_api as client;
    #[cfg(feature = "server")]
    pub use self::generated::server::c_api as server;

    mod generated {
        #[cfg(feature = "client")]
        pub mod client {
            pub mod c_interfaces {
                pub(crate) use wayland_sys::common::{wl_argument, wl_interface};
                pub(crate) use wayland_client::sys::protocol_interfaces::{wl_surface_interface, wl_output_interface};
                pub(crate) use wayland_protocols::xdg_shell::c_interfaces::xdg_popup_interface;
                include!(concat!(env!("OUT_DIR"), "/layer-shell-unstable-v1-interfaces.rs"));
            }

            pub mod c_api {
                pub(crate) use wayland_sys as sys;
                pub(crate) use wayland_client::{NewProxy, Proxy};
                pub(crate) use wayland_commons::{AnonymousObject, Interface, MessageGroup};
                pub(crate) use wayland_client::protocol::{wl_surface, wl_output};
                pub(crate) use wayland_protocols::xdg_shell::client::xdg_popup;
                include!(concat!(env!("OUT_DIR"), "/layer-shell-unstable-v1-client.rs"));
            }
        }

        #[cfg(feature = "server")]
        pub mod server {
            pub mod c_interfaces {
                pub(crate) use wayland_sys::common::{wl_argument, wl_interface};
                pub(crate) use wayland_server::sys::protocol_interfaces::{wl_surface_interface, wl_output_interface};
                pub(crate) use wayland_protocols::xdg_shell::c_interfaces::xdg_popup_interface;
                include!(concat!(env!("OUT_DIR"), "/layer-shell-unstable-v1-interfaces.rs"));
            }

            pub mod c_api {
                pub(crate) use wayland_sys as sys;
                pub(crate) use wayland_server::{NewResource, Resource};
                pub(crate) use wayland_commons::{AnonymousObject, Interface, MessageGroup};
                pub(crate) use wayland_server::protocol::{wl_surface, wl_output};
                pub(crate) use wayland_protocols::xdg_shell::server::xdg_popup;
                include!(concat!(env!("OUT_DIR"), "/layer-shell-unstable-v1-server.rs"));
            }
        }
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy))]
#[allow(dead_code,non_camel_case_types,unused_unsafe,unused_variables)]
#[allow(non_upper_case_globals,non_snake_case,unused_imports,missing_docs)]
pub mod dank_private {
    #[cfg(feature = "client")]
    pub use self::generated::client::c_api as client;
    #[cfg(feature = "server")]
    pub use self::generated::server::c_api as server;

    mod generated {
        #[cfg(feature = "client")]
        pub mod client {
            pub mod c_interfaces {
                pub(crate) use wayland_sys::common::{wl_argument, wl_interface};
                include!(concat!(env!("OUT_DIR"), "/dank-shell-private-api-interfaces.rs"));
            }

            pub mod c_api {
                pub(crate) use wayland_sys as sys;
                pub(crate) use wayland_client::{NewProxy, Proxy};
                pub(crate) use wayland_commons::{AnonymousObject, Interface, MessageGroup};
                include!(concat!(env!("OUT_DIR"), "/dank-shell-private-api-client.rs"));
            }
        }

        #[cfg(feature = "server")]
        pub mod server {
            pub mod c_interfaces {
                pub(crate) use wayland_sys::common::{wl_argument, wl_interface};
                include!(concat!(env!("OUT_DIR"), "/dank-shell-private-api-interfaces.rs"));
            }

            pub mod c_api {
                pub(crate) use wayland_sys as sys;
                pub(crate) use wayland_server::{NewResource, Resource};
                pub(crate) use wayland_commons::{AnonymousObject, Interface, MessageGroup};
                include!(concat!(env!("OUT_DIR"), "/dank-shell-private-api-server.rs"));
            }
        }
    }
}

pub trait CborConv: Sized + Serialize + DeserializeOwned {
    fn from_cbor(data: &[u8]) -> serde_cbor::error::Result<Self> {
        serde_cbor::from_slice(data)
    }
    fn to_cbor(self) -> serde_cbor::error::Result<Vec<u8>> {
        serde_cbor::to_vec(&self)
    }
}

pub mod permissions;
pub mod outputs;

#[cfg(feature = "gtkclient")]
pub mod gtkclient;
