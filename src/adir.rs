extern crate clap;
extern crate gtk;

use gtk::{
    BoxExt, ContainerExt, GtkWindowExt, HeaderBarExt, ObjectExt, RangeExt, ScaleExt,
    StyleContextExt, WidgetExt,
};
use std::process;

use clap::{App, Arg, ArgMatches, SubCommand};

/// A struct that holds the shared configuration options.
struct BaseConfig {
    width: i32,
    height: i32,
    posx: i32,
    posy: i32,
    close_unfocus: bool,
    class: String,
}

/// A struct that holds the configuration options used by the Scale dialog.
struct ScaleConfig {
    val: i32,
    max_val: i32,
    min_val: i32,
    step: i32,
    mark: Option<(i32, String)>,
    orientation: gtk::Orientation,
    hide_value: bool,
}

/// A struct that holds the configuration options used by the Calendar dialog.
struct CalendarConfig {
    _dummy: i32,
}

enum WidgetConfig {
    Scale(ScaleConfig),
    Calendar(CalendarConfig),
}

/// A struct that holds the base and widget-specific configurations.
struct Config {
    base_config: BaseConfig,
    widget_config: WidgetConfig,
}

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

impl Config {
    pub fn new(base_config: BaseConfig, widget_config: WidgetConfig) -> Config {
        Config {
            base_config,
            widget_config,
        }
    }

    pub fn width(&self) -> i32 {
        self.base_config.width
    }

    pub fn height(&self) -> i32 {
        self.base_config.height
    }
}

impl BaseConfig {
    pub fn new(matches: &ArgMatches) -> Result<BaseConfig, String> {
        let width = matches
            .value_of("width")
            .unwrap_or("0")
            .parse::<i32>()
            .or(Err("Width is not an integer"))?;
        let height = matches
            .value_of("height")
            .unwrap_or("0")
            .parse::<i32>()
            .or(Err("Height is not an integer"))?;

        let posx = matches
            .value_of("posx")
            .expect("Argument with default value `posx', not specified")
            .parse::<i32>()
            .or(Err("posx is not an integer"))?;
        let posy = matches
            .value_of("posy")
            .expect("Argument with default value `posy', not specified")
            .parse::<i32>()
            .or(Err("posy is not an integer"))?;

        let class = matches.value_of("class").unwrap_or("").to_string();

        let close_unfocus = matches.occurrences_of("close-unfocus") == 1;

        Ok(BaseConfig {
            class,
            width,
            height,
            posx,
            posy,
            close_unfocus,
        })
    }
}

impl CalendarConfig {
    pub fn new(_matches: &ArgMatches) -> CalendarConfig {
        CalendarConfig { _dummy: 3 }
    }
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
    fn new(config: &Config) -> Application {
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
        window.set_wmclass("app-name", config.base_config.class.as_str());
        // The icon the app will display.
        gtk::Window::set_default_icon_name("iconname");

        // Add the content box into the window.
        window.add(&content.container);
        window.set_default_size(config.width(), config.height());
        window.move_(config.base_config.posx, config.base_config.posy);

        // Programs what to do when the exit button is used.
        window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            gtk::Inhibit(false)
        });

        if config.base_config.close_unfocus {
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
}

impl Content {
    fn new(config: &Config) -> Content {
        // Create a vertical box to store all of it's inner children vertically.
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        match config.widget_config {
            WidgetConfig::Scale(ref scale_conf) => {
                let scale = gtk::Scale::new_with_range(
                    scale_conf.orientation,
                    scale_conf.min_val as f64,
                    scale_conf.max_val as f64,
                    scale_conf.step as f64,
                );
                scale.set_value(scale_conf.val as f64);
                if scale_conf.hide_value {
                    scale.set_draw_value(false);
                }
                if let Some((pos, ref text)) = scale_conf.mark {
                    scale.add_mark(pos as f64, gtk::PositionType::Left, text.as_str());
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
            WidgetConfig::Calendar(ref _calendar_config) => {
                let calendar = gtk::Calendar::new();
                container.pack_start(&calendar, true, true, 0);
            }
        }

        Content { container }
    }
}

impl ScaleConfig {
    pub fn new(matches: &ArgMatches) -> Result<ScaleConfig, String> {
        let max_val = matches
            .value_of("maxval")
            .expect("Argument with default value `max-value', not specified")
            .parse::<i32>()
            .or(Err("max-value is not an integer"))?;

        let min_val = matches
            .value_of("minval")
            .expect("Argument with default value `min-value', not specified")
            .parse::<i32>()
            .or(Err("min-value is not an integer"))?;

        let val = matches
            .value_of("val")
            .expect("Argument with default value `value', not specified")
            .parse::<i32>()
            .or(Err("value is not an integer"))?;

        let step = matches
            .value_of("step")
            .expect("Argument with default value `step', not specified")
            .parse::<i32>()
            .or(Err("step is not an integer"))?;

        let orientation = if matches.occurrences_of("vertical") == 1 {
            gtk::Orientation::Vertical
        } else {
            gtk::Orientation::Horizontal
        };

        let hide_value = matches.occurrences_of("hide-val") == 1;

        let mark = match matches.value_of("mark") {
            None => None,
            Some(s) => {
                let mut split = s.split(":");
                let text = split.next().unwrap_or("").to_string();
                let pos = split
                    .next()
                    .ok_or("mark argument provided but not position")?
                    .parse::<i32>()
                    .or(Err("not a valid position for a mark"))?;
                Some((pos, text))
            }
        };

        Ok(ScaleConfig {
            val,
            max_val,
            min_val,
            step,
            mark,
            orientation,
            hide_value,
        })
    }
}

fn create_parser() -> App<'static, 'static> {
    App::new("adir")
        .version("0.1")
        .author("mandragore")
        .about("Creates dialog windows")
        .subcommand(SubCommand::with_name("calendar").about("Opens a calendar dialog"))
        .subcommand(
            SubCommand::with_name("scale")
                .about("Opens a scale dialog")
                .arg(
                    Arg::with_name("mark")
                        .long("--mark")
                        .require_equals(true)
                        .value_name("MARK")
                        .help("Place a mark on the scale")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("maxval")
                        .long("--max-value")
                        .require_equals(true)
                        .default_value("100")
                        .value_name("MAXVAL")
                        .help("The maximum value of the scale")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("minval")
                        .long("--min-value")
                        .require_equals(true)
                        .default_value("0")
                        .value_name("MINVAL")
                        .help("The minimum value of the scale")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("val")
                        .long("--value")
                        .require_equals(true)
                        .default_value("0")
                        .value_name("VALUE")
                        .help("The initial value of the scale")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("step")
                        .long("--step")
                        .require_equals(true)
                        .default_value("1")
                        .value_name("STEP")
                        .help("The step of the scale")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("vertical")
                        .long("--vertical")
                        .help("Vertically orient the scale")
                        .takes_value(false)
                        .multiple(false),
                )
                .arg(
                    Arg::with_name("invert")
                        .long("--invert")
                        .help("Invert scale direction")
                        .takes_value(false)
                        .multiple(false),
                )
                .arg(
                    Arg::with_name("hide-val")
                        .long("--hide-value")
                        .help("Hide value")
                        .takes_value(false)
                        .multiple(false),
                ),
        )
        .arg(
            Arg::with_name("class")
                .long("--class")
                .help("Set the dialog window class")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("width")
                .long("--width")
                .help("Set the dialog window width")
                .takes_value(true)
                .require_equals(true),
        )
        .arg(
            Arg::with_name("height")
                .long("--height")
                .help("Set the dialog window height")
                .require_equals(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("posx")
                .long("--posx")
                .help("Set the x position of the window")
                .require_equals(true)
                .takes_value(true)
                .default_value("0"),
        )
        .arg(
            Arg::with_name("posy")
                .long("--posy")
                .help("Set the y position of the window")
                .require_equals(true)
                .takes_value(true)
                .default_value("0"),
        )
        .arg(
            Arg::with_name("close-unfocus")
                .long("--close-on-unfocus")
                .help("Close the dialog window when it looses focus")
                .takes_value(false)
                .multiple(false),
        )
}

fn main() -> Result<(), String> {
    let parser = create_parser();
    let matches = parser.get_matches();
    // Initialize GTK before proceeding.
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    let (widget_name, submatches) = matches.subcommand();
    let submatches = submatches.ok_or("Command not specified")?;
    let base_config = BaseConfig::new(&matches)?;
    let config = match widget_name {
        "scale" => {
            let scale_config = ScaleConfig::new(submatches)?;
            let widget_config = WidgetConfig::Scale(scale_config);
            Config::new(base_config, widget_config)
        }
        "calendar" => {
            let calendar_config = CalendarConfig::new(submatches);
            let widget_config = WidgetConfig::Calendar(calendar_config);
            Config::new(base_config, widget_config)
        }
        _ => return Err(String::from("Unknown dialog type")),
    };

    // Initialize the UI's initial state
    let app = Application::new(&config);

    // Make all the widgets within the UI visible.
    app.window.show_all();

    // Start the GTK main event loop
    gtk::main();
    Ok(())
}
