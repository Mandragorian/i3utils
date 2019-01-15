use clap::{App, Arg, SubCommand};

use i3utils::gtk::widgets::config::{
    BaseConfig, CalendarConfig, Config, ScaleConfig, WidgetConfig,
};
use i3utils::gtk::Application;

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
    gtk::init().or(Err("Failed to initialize GTK application"))?;

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
    app.show_all();

    // Start the GTK main event loop
    gtk::main();
    Ok(())
}
