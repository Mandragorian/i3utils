use clap::ArgMatches;

/// A struct that holds the shared configuration options.
pub struct BaseConfig {
    width: i32,
    height: i32,
    posx: i32,
    posy: i32,
    close_unfocus: bool,
    class: String,
}

/// A struct that holds the configuration options used by the Scale dialog.
pub struct ScaleConfig {
    val: i32,
    max_val: i32,
    min_val: i32,
    step: i32,
    mark: Option<(i32, String)>,
    orientation: gtk::Orientation,
    hide_value: bool,
}

/// A struct that holds the configuration options used by the Calendar dialog.
pub struct CalendarConfig {
    _dummy: i32,
}

pub enum WidgetConfig {
    Scale(ScaleConfig),
    Calendar(CalendarConfig),
}

/// A struct that holds the base and widget-specific configurations.
pub struct Config {
    base_config: BaseConfig,
    widget_config: WidgetConfig,
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

    pub fn class(&self) -> &String {
        &self.base_config.class
    }

    pub fn posx(&self) -> i32 {
        self.base_config.posx
    }

    pub fn posy(&self) -> i32 {
        self.base_config.posy
    }

    pub fn close_unfocus(&self) -> bool {
        self.base_config.close_unfocus
    }

    pub fn widget_config(&self) -> &WidgetConfig {
        &self.widget_config
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

    pub fn orientation(&self)  { 
    }
}

impl CalendarConfig {
    pub fn new(_matches: &ArgMatches) -> CalendarConfig {
        CalendarConfig { _dummy: 3 }
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

    pub fn val(&self) -> i32 {
        self.val
    }

    pub fn min_val(&self) -> i32 {
        self.min_val
    }

    pub fn max_val(&self) -> i32 {
        self.max_val
    }

    pub fn orientation(&self) -> gtk::Orientation {
        self.orientation
    }

    pub fn step(&self) -> i32 {
        self.step
    }

    pub fn hide_value(&self) -> bool {
        self.hide_value
    }

    pub fn mark(&self) -> &Option<(i32, String)> {
        &self.mark
    }
}
