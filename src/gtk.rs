pub mod widgets;

use gtk::{
    BoxExt, ContainerExt, GtkWindowExt, HeaderBarExt, ObjectExt, RangeExt, ScaleExt,
    StyleContextExt, WidgetExt,
};

use self::widgets::config::{Config, WidgetConfig};

pub struct Application {
    pub window: gtk::Window,
    pub content: Content,
}

pub struct Header {
    pub container: gtk::HeaderBar,
    pub hit: gtk::Button,
    pub heal: gtk::Button,
}

pub struct Content {
    pub container: gtk::Box,
}

impl Header {
    fn new() -> Header {
        // Creates the main header bar container widget.
        let container = gtk::HeaderBar::new();

        // Sets the text to display in the title section of the header bar.
        container.set_title("adir");
        // Enable the window controls within this headerbar.
        container.set_show_close_button(true);

        // Create the hit and heal buttons.
        let hit = gtk::Button::new_with_label("Hit");
        let heal = gtk::Button::new_with_label("Heal");

        // Add the corresponding style classes to those buttons.
        hit.get_style_context()
            .map(|c| c.add_class("destructive-action"));
        heal.get_style_context()
            .map(|c| c.add_class("suggested-action"));

        // THen add them to the header bar.
        container.pack_start(&hit);
        container.pack_end(&heal);

        // Returns the header and all of it's state
        Header {
            container,
            hit,
            heal,
        }
    }
}

impl Application {
    pub fn new(config: &Config) -> Application {
        // Create a new top level window.
        let window = gtk::Window::new(gtk::WindowType::Toplevel);

        //// Create a the headerbar and it's associated content.
        //let header = Header::new();
        //// Set the headerbar as the title bar widget.
        //window.set_titlebar(&header.container);

        // Contains the content within the window.
        let content = Content::new(config);

        // Set the title of the window.
        window.set_title("adir");
        // Set the window manager class.
        window.set_wmclass("app-name", config.class().as_str());
        // The icon the app will display.
        gtk::Window::set_default_icon_name("iconname");

        // Add the content box into the window.
        window.add(&content.container);
        window.set_default_size(config.width(), config.height());
        window.move_(config.posx(), config.posy());

        // Programs what to do when the exit button is used.
        window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            gtk::Inhibit(false)
        });

        if config.close_unfocus() {
            window
                .connect("focus-out-event", false, |_| {
                    gtk::main_quit();
                    let v = gtk::Type::Bool;
                    Some(gtk::Value::from_type(v))
                })
                .unwrap();
        }

        // Return our main application state
        Application {
            window,
            //header,
            content,
        }
    }

    pub fn show_all(&self) {
        self.window.show_all();
    }
}

impl Content {
    fn new(config: &Config) -> Content {
        // Create a vertical box to store all of it's inner children vertically.
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        match config.widget_config() {
            widgets::config::WidgetConfig::Scale(scale_conf) => {
                let scale = gtk::Scale::new_with_range(
                    scale_conf.orientation(),
                    scale_conf.min_val() as f64,
                    scale_conf.max_val() as f64,
                    scale_conf.step() as f64,
                );
                scale.set_value(scale_conf.val() as f64);
                if scale_conf.hide_value() {
                    scale.set_draw_value(false);
                }
                if let Some((pos, text)) = scale_conf.mark() {
                    scale.add_mark(*pos as f64, gtk::PositionType::Left, text.as_str());
                }
                container.pack_start(&scale, true, true, 0);
                scale
                    .clone()
                    .connect("value-changed", false, |x| {
                        let val: gtk::Scale = x[0].get().unwrap();
                        println!("{}", val.get_value() as i32);
                        None
                    })
                    .unwrap();
            }
            WidgetConfig::Calendar(_calendar_config) => {
                let calendar = gtk::Calendar::new();
                container.pack_start(&calendar, true, true, 0);
            }
        }

        Content { container }
    }
}
