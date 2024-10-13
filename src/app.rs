use std::collections::HashMap;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;
use ratatui::style::Color;

#[derive(Deserialize, Default, Debug)]
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

#[derive(Deserialize, Default, Debug)]
pub struct Main {
    temp: f32,
    humidity: f32,
    pressure: f32,
}

#[derive(Deserialize, Default, Debug)]
pub struct Wind {
    speed: f32,
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
        city,
        country_code,
        api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

pub fn display_weather_info(weather: &WeatherResponse) -> String {
    let description = &weather.weather[0].description;
    let temperature = weather.main.temp;
    let humidity = weather.main.humidity;
    let pressure = weather.main.pressure;
    let wind_speed = weather.wind.speed;

    format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C,
        > Humidity: {:.1}%,
        > Pressure: {:.1} hPa,
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

fn get_weather_text_colour(description: String) -> Color {
    let color = match description.as_str() {
        "clear sky" => Color::Blue,
        "few clouds" | "scattered clouds" | "broken clouds" => Color::Yellow,
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => Color::Green,
        "shower rain" | "rain" | "thunderstorm" | "snow" => Color::Gray,
        _ => Color::White,
    };

    color
}

fn get_temp_emoji(temperature: f32) -> &'static str{
    if temperature < 0.0 {
        "🥶"
    } else if temperature >= 0.0 && temperature < 15.0 {
        "☁️"
    } else if temperature >= 15.0 && temperature < 25.0 {
        "⛅️"
    } else if temperature >= 25.0 && temperature < 35.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

pub struct App {
    pub country_input: String,
    pub city_input: String,
    pub countries: Vec<String>,
    pub cities: Vec<String>,
    pub weather: Vec<WeatherResponse>,
    pub color: Vec<Color>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> App {
        App {
            country_input: String::new(),
            city_input: String::new(),
            countries: Vec::new(),
            cities: Vec::new(),
            weather: Vec::new(),
            color: Vec::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_country_city(&mut self) {
        dotenv().ok();

        let city: &str = self.city_input.trim();
        let country_code: &str = self.country_input.trim();
        let api_key: &str = &env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set");

        self.countries.push(self.country_input.clone());
        self.cities.push(self.city_input.clone());
        let weather_info = get_weather_info(&city, &country_code, api_key);
        match weather_info {
            Ok(response) => {
                self.color.push(get_weather_text_colour(response.weather[0].description.clone()));
                self.weather.push(response);
            }
            Err(err) => {
                self.color.push(Color::Red);
                self.weather.push(WeatherResponse::default());
                eprintln!("Error: {}", err);
            }
        }

        self.country_input = String::new();
        self.city_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Country => self.currently_editing = Some(CurrentlyEditing::City),
                CurrentlyEditing::City => self.currently_editing = Some(CurrentlyEditing::Country),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Country);
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        Ok(())
    }
}
