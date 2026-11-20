use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

use crate::config::app_constants::{
    DEFAULT_CONCURRENCY, DEFAULT_DARK_MODE, DEFAULT_LISTEN_ONLY, DEFAULT_REFRESH_INTERVAL_SECS,
    DEFAULT_TIMEOUT_MS, DEFAULT_UI_SCALE, ENV_CONCURRENCY_KEY, ENV_DARK_MODE_KEY,
    ENV_LISTEN_ONLY_KEY, ENV_LOCAL_FILENAME, ENV_REFRESH_INTERVAL_KEY, ENV_TIMEOUT_MS_KEY,
    ENV_UI_SCALE_KEY, MAX_CONCURRENCY, MAX_REFRESH_INTERVAL_SECS, MAX_TIMEOUT_MS, MAX_UI_SCALE,
    MIN_CONCURRENCY, MIN_REFRESH_INTERVAL_SECS, MIN_TIMEOUT_MS, MIN_UI_SCALE,
};
use crate::utils::input_validator::{validateConcurrency, validateTimeoutMs};

// Cấu hình khởi chạy và tùy chỉnh bền vững của ứng dụng
#[derive(Debug, Clone)]
pub struct AppEnvConfig {
    pub refreshIntervalSecs: u64,
    pub defaultTimeoutMs: u64,
    pub defaultConcurrency: usize,
    pub defaultDarkMode: bool,
    pub defaultUiScale: f32,
    pub defaultListenOnly: bool,
}

impl Default for AppEnvConfig {
    fn default() -> Self {
        Self {
            refreshIntervalSecs: DEFAULT_REFRESH_INTERVAL_SECS,
            defaultTimeoutMs: DEFAULT_TIMEOUT_MS,
            defaultConcurrency: DEFAULT_CONCURRENCY,
            defaultDarkMode: DEFAULT_DARK_MODE,
            defaultUiScale: DEFAULT_UI_SCALE,
            defaultListenOnly: DEFAULT_LISTEN_ONLY,
        }
    }
}

impl AppEnvConfig {
    // Nạp cấu hình từ file .env.local (nếu tồn tại) kết hợp biến môi trường hiện hành
    pub fn loadFromEnv() -> Self {
        let _ = dotenvy::from_filename(ENV_LOCAL_FILENAME);
        Self::fromMapOpt(None)
    }

    // Đọc cấu hình trực tiếp từ một đường dẫn file env cụ thể (phục vụ lưu & kiểm thử)
    pub fn loadFromFile(filePath: &Path) -> Self {
        let mut pairs = HashMap::new();
        if let Ok(iter) = dotenvy::from_path_iter(filePath) {
            for item in iter.flatten() {
                pairs.insert(item.0, item.1);
            }
        }
        Self::fromMapOpt(Some(&pairs))
    }

    // So sánh hai cấu hình có giống nhau hay không (xử lý sai số dấu phẩy động ở uiScale)
    pub fn matchesConfig(&self, other: &Self) -> bool {
        self.refreshIntervalSecs == other.refreshIntervalSecs
            && self.defaultTimeoutMs == other.defaultTimeoutMs
            && self.defaultConcurrency == other.defaultConcurrency
            && self.defaultDarkMode == other.defaultDarkMode
            && self.defaultListenOnly == other.defaultListenOnly
            && (self.defaultUiScale - other.defaultUiScale).abs() < 0.005
    }

    // Kiểm tra hợp lệ và ghi cấu hình vào file .env.local của dự án
    pub fn saveToEnvLocal(&self) -> Result<(), String> {
        self.saveToFile(Path::new(ENV_LOCAL_FILENAME))
    }

    // Cập nhật các khóa cấu hình ứng dụng vào file đích nhưng giữ nguyên các biến khác của người dùng
    pub fn saveToFile(&self, filePath: &Path) -> Result<(), String> {
        if self.refreshIntervalSecs < MIN_REFRESH_INTERVAL_SECS
            || self.refreshIntervalSecs > MAX_REFRESH_INTERVAL_SECS
        {
            return Err(format!(
                "Chu kỳ làm mới phải từ {} đến {} giây.",
                MIN_REFRESH_INTERVAL_SECS, MAX_REFRESH_INTERVAL_SECS
            ));
        }
        validateTimeoutMs(self.defaultTimeoutMs)?;
        validateConcurrency(self.defaultConcurrency)?;
        if !self.defaultUiScale.is_finite()
            || self.defaultUiScale < MIN_UI_SCALE
            || self.defaultUiScale > MAX_UI_SCALE
        {
            return Err(format!(
                "Tỷ lệ hiển thị phải từ {:.0}% đến {:.0}%.",
                MIN_UI_SCALE * 100.0,
                MAX_UI_SCALE * 100.0
            ));
        }

        let managedPairs = [
            (ENV_REFRESH_INTERVAL_KEY, self.refreshIntervalSecs.to_string()),
            (ENV_TIMEOUT_MS_KEY, self.defaultTimeoutMs.to_string()),
            (ENV_CONCURRENCY_KEY, self.defaultConcurrency.to_string()),
            (ENV_DARK_MODE_KEY, self.defaultDarkMode.to_string()),
            (ENV_UI_SCALE_KEY, format!("{:.2}", self.defaultUiScale)),
            (ENV_LISTEN_ONLY_KEY, self.defaultListenOnly.to_string()),
        ];

        let mut updatedFlags = [false; 6];
        let mut outputLines: Vec<String> = Vec::new();

        if let Ok(existingText) = fs::read_to_string(filePath) {
            for rawLine in existingText.lines() {
                let trimmed = rawLine.trim();
                let mut replaced = false;
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    if let Some((keyPart, _)) = trimmed.split_once('=') {
                        let keyName = keyPart.trim();
                        for (idx, (managedKey, managedVal)) in managedPairs.iter().enumerate() {
                            if keyName == *managedKey {
                                outputLines.push(format!("{}={}", managedKey, managedVal));
                                updatedFlags[idx] = true;
                                replaced = true;
                                break;
                            }
                        }
                    }
                }
                if !replaced {
                    outputLines.push(rawLine.to_string());
                }
            }
        }

        for (idx, (managedKey, managedVal)) in managedPairs.iter().enumerate() {
            if !updatedFlags[idx] {
                outputLines.push(format!("{}={}", managedKey, managedVal));
            }
        }

        let mut content = outputLines.join("\n");
        if !content.is_empty() {
            content.push('\n');
        }
        fs::write(filePath, content)
            .map_err(|err| format!("Không thể lưu file cấu hình '{}': {}", filePath.display(), err))
    }

    // Dựng AppEnvConfig từ map đã đọc hoặc từ biến môi trường của tiến trình
    fn fromMapOpt(sourceMap: Option<&HashMap<String, String>>) -> Self {
        Self {
            refreshIntervalSecs: readClamped(
                sourceMap,
                ENV_REFRESH_INTERVAL_KEY,
                DEFAULT_REFRESH_INTERVAL_SECS,
                MIN_REFRESH_INTERVAL_SECS,
                MAX_REFRESH_INTERVAL_SECS,
            ),
            defaultTimeoutMs: readClamped(
                sourceMap,
                ENV_TIMEOUT_MS_KEY,
                DEFAULT_TIMEOUT_MS,
                MIN_TIMEOUT_MS,
                MAX_TIMEOUT_MS,
            ),
            defaultConcurrency: readClamped(
                sourceMap,
                ENV_CONCURRENCY_KEY,
                DEFAULT_CONCURRENCY,
                MIN_CONCURRENCY,
                MAX_CONCURRENCY,
            ),
            defaultDarkMode: readBoolFlag(sourceMap, ENV_DARK_MODE_KEY, DEFAULT_DARK_MODE),
            defaultUiScale: readClamped(
                sourceMap,
                ENV_UI_SCALE_KEY,
                DEFAULT_UI_SCALE,
                MIN_UI_SCALE,
                MAX_UI_SCALE,
            ),
            defaultListenOnly: readBoolFlag(sourceMap, ENV_LISTEN_ONLY_KEY, DEFAULT_LISTEN_ONLY),
        }
    }
}

// Đọc chuỗi giá trị theo khóa từ map hoặc biến môi trường hệ thống
fn readRawValue(sourceMap: Option<&HashMap<String, String>>, envKey: &str) -> Option<String> {
    match sourceMap {
        Some(map) => map.get(envKey).cloned(),
        None => env::var(envKey).ok(),
    }
}

// Đọc giá trị số bất kỳ (u64, usize, f32) và giới hạn an toàn trong khoảng [minValue, maxValue]
fn readClamped<T>(
    sourceMap: Option<&HashMap<String, String>>,
    envKey: &str,
    fallbackValue: T,
    minValue: T,
    maxValue: T,
) -> T
where
    T: std::str::FromStr + PartialOrd + Copy,
{
    let Some(rawVal) = readRawValue(sourceMap, envKey) else {
        return fallbackValue;
    };
    let Ok(parsedVal) = rawVal.trim().parse::<T>() else {
        return fallbackValue;
    };
    // So sánh tường minh với cả hai biên để loại bỏ trường hợp NaN với kiểu số thực
    if !(parsedVal >= minValue && parsedVal <= maxValue) {
        if parsedVal < minValue {
            minValue
        } else if parsedVal > maxValue {
            maxValue
        } else {
            fallbackValue
        }
    } else {
        parsedVal
    }
}

// Đọc giá trị boolean từ map hoặc biến môi trường
fn readBoolFlag(
    sourceMap: Option<&HashMap<String, String>>,
    envKey: &str,
    fallbackValue: bool,
) -> bool {
    let Some(rawVal) = readRawValue(sourceMap, envKey) else {
        return fallbackValue;
    };
    match rawVal.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" => true,
        "false" | "0" | "no" => false,
        _ => fallbackValue,
    }
}
