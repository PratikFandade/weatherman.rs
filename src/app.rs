use std::env;
use dotenv::dotenv;
use ratatui::style::Color;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    temp: f32,
    humidity: f32,
    pressure: f32,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    speed: f32,
}

impl Default for WeatherResponse {
    fn default() -> Self {
        Self {
            weather: vec![Weather::default()],
            main: Main::default(),
            wind: Wind::default(),
            name: String::new(),
        }
    }
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            description: String::from("ERR: Check the input again"),
        }
    }
}

impl Default for Main {
    fn default() -> Self {
        Self {
            temp: 0.0,
            humidity: 0.0,
            pressure: 0.0,
        }
    }
}

impl Default for Wind {
    fn default() -> Self {
        Self {
            speed: 0.0,
        }
    }
}

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Country,
    City,
}

fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    response.json::<WeatherResponse>()
}

pub fn display_weather_info(weather: &WeatherResponse) -> String {
    let description = &weather.weather[0].description;
    let temperature = weather.main.temp;
    let humidity = weather.main.humidity;
    let pressure = weather.main.pressure;
    let wind_speed = weather.wind.speed;

    format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C
        > Humidity: {:.1}%
        > Pressure: {:.1} hPa
        > Wind Speed: {:.1} m/s",
        weather.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed
    )
}

fn get_weather_text_color(description: &str) -> Color {
    match description {
        "clear sky" => Color::Blue,
        "few clouds" | "scattered clouds" | "broken clouds" => Color::Yellow,
        "overcast clouds" | "mist" | "haze" | "smoke" | "fog" => Color::Green,
        "shower rain" | "rain" | "thunderstorm" | "snow" => Color::Gray,
        _ => Color::White,
    }
}

fn get_temp_emoji(temperature: f32) -> &'static str {
    match temperature {
        temp if temp < 0.0 => "🥶",
        temp if temp >= 0.0 && temp < 15.0 => "☁️",
        temp if temp >= 15.0 && temp < 25.0 => "⛅️",
        temp if temp >= 25.0 && temp < 35.0 => "🌤️",
        _ => "🔥",
    }
}

pub struct App {
    pub country_input: String,
    pub city_input: String,
    pub countries: Vec<String>,
    pub cities: Vec<String>,
    pub weather: Vec<WeatherResponse>,
    pub colors: Vec<Color>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> Self {
        Self {
            country_input: String::new(),
            city_input: String::new(),
            countries: Vec::with_capacity(10),
            cities: Vec::with_capacity(10),
            weather: Vec::with_capacity(10),
            colors: Vec::with_capacity(10),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_country_city(&mut self) {
        dotenv().ok();

        let city = self.city_input.trim();
        let country_code = self.country_input.trim();
        let api_key = env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set");

        self.countries.push(self.country_input.clone());
        self.cities.push(self.city_input.clone());

        match get_weather_info(city, country_code, &api_key) {
            Ok(response) => {
                self.colors.push(get_weather_text_color(&response.weather[0].description));
                self.weather.push(response);
            }
            Err(_err) => {
                self.colors.push(Color::Red);
                self.weather.push(WeatherResponse::default());
            }
        }

        self.country_input.clear();
        self.city_input.clear();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        self.currently_editing = match self.currently_editing {
            Some(CurrentlyEditing::Country) => Some(CurrentlyEditing::City),
            Some(CurrentlyEditing::City) => Some(CurrentlyEditing::Country),
            None => Some(CurrentlyEditing::Country),
        };
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        Ok(())
    }
}
