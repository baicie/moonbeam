use moonbeam::blocking::Brightness;

#[tauri::command]
pub fn get_brightness() -> Result<f64, String> {
    let devices = moonbeam::blocking::brightness_devices();
    let device = devices
        .filter_map(Result::ok)
        .next()
        .ok_or_else(|| "No display found".to_string())?;

    device
        .get()
        .map(|value| value as f64 / 100.0)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_brightness(value: f64) -> Result<(), String> {
    if !(0.0..=1.0).contains(&value) {
        return Err("Brightness value must be between 0.0 and 1.0".to_string());
    }

    let devices = moonbeam::blocking::brightness_devices();
    let device = devices
        .filter_map(Result::ok)
        .next()
        .ok_or_else(|| "No display found".to_string())?;

    device
        .set((value * 100.0) as u32)
        .map_err(|e| e.to_string())
}
