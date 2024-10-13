# Weather Station 🌤️

**Weather Station** is a command-line application written in Rust that provides real-time weather information for any city in the world. It uses the OpenWeatherMap API to fetch weather data and displays it in a user-friendly format with colored output.

![Demo](./images/demo.gif)

## Features ✨

- Retrieve current weather conditions like temperature, humidity, pressure, and wind speed.
- Beautifully formatted and colored text output for an enhanced terminal experience.
- Support for emojis to visualize temperature ranges.
- Loop-based user interaction to search for weather in multiple cities without restarting the app.
  
## Prerequisites 📦

- **Rust**: Make sure you have Rust installed on your system. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).
- **Cargo**: Cargo is the Rust package manager, which is included with the Rust installation.
- **OpenWeatherMap API Key**: Sign up on [OpenWeatherMap](https://openweathermap.org/) to get a free API key.

## Setup 🔧

1. Clone the repository:

   ```bash
   git clone https://github.com/pratikfandade/weather-cli.git
   cd weather-station
   ```

2. Add add the `OPENWEATHERMAP_API_KEY` in the file named `.env` to your project root:

   ```bash
   cat .env.example | sed 's/OPENWEATHERMAP_API_KEY=changeme/OPENWEATHERMAP_API_KEY=YOUR_API_KEY/g' > .env
   ```

3. Run the program:

   ```bash
   cargo run
   ```

## Contributing 🌱

If you have any suggestions or find any bugs, please [open an issue](https://github.com/pratikfandade/weather-cli/issues). I'll be happy to help!
