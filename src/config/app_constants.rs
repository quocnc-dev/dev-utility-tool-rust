// Hằng số cấu hình giao diện, kích thước cửa sổ và tỷ lệ hiển thị
pub const APP_TITLE: &str = "DevUtility Hub";
pub const APP_SUBTITLE: &str = "Bộ công cụ lập trình đa nền tảng";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const WINDOW_DEFAULT_WIDTH: f32 = 1380.0;
pub const WINDOW_DEFAULT_HEIGHT: f32 = 840.0;
pub const WINDOW_MIN_WIDTH: f32 = 1040.0;
pub const WINDOW_MIN_HEIGHT: f32 = 660.0;
pub const SIDEBAR_DEFAULT_WIDTH: f32 = 290.0;
pub const SIDEBAR_MIN_WIDTH: f32 = 250.0;
pub const SIDEBAR_MAX_WIDTH: f32 = 360.0;
pub const SIDEBAR_INPUT_HEIGHT: f32 = 34.0;
pub const SIDEBAR_SEARCH_MIN_WIDTH: f32 = 120.0;
pub const SIDEBAR_CLEAR_RESERVE_WIDTH: f32 = 68.0;
pub const NAV_ITEM_HEIGHT: f32 = 54.0;
pub const NAV_DOT_OFFSET_X: f32 = 16.0;
pub const NAV_DOT_RADIUS: f32 = 4.0;
pub const NAV_TEXT_OFFSET_X: f32 = 30.0;
pub const NAV_TITLE_OFFSET_Y: f32 = 10.0;
pub const NAV_TAG_OFFSET_Y: f32 = 31.0;
pub const TABLE_HEADER_HEIGHT: f32 = 40.0;
pub const TABLE_ROW_HEIGHT: f32 = 42.0;
pub const MIN_INTERACT_WIDTH: f32 = 56.0;
pub const MIN_INTERACT_HEIGHT: f32 = 36.0;
pub const ACTION_BTN_MIN_HEIGHT: f32 = 32.0;
pub const PROGRESS_BAR_HEIGHT: f32 = 24.0;
pub const MODAL_MAX_WIDTH: f32 = 460.0;
pub const DEFAULT_REPAINT_INTERVAL_MS: u64 = 120;
pub const TEXT_AREA_MIN_ROWS: usize = 14;
pub const JSON_SAMPLE_OUTPUT_ROWS: usize = 14;
pub const SETTINGS_LABEL_COL_WIDTH: f32 = 320.0;

// Thang cỡ chữ chuẩn hóa theo chuẩn đọc rõ ràng cho công cụ thao tác (Operate UI)
pub const FONT_SIZE_SMALL: f32 = 13.0;
pub const FONT_SIZE_MONOSPACE: f32 = 14.5;
pub const FONT_SIZE_BODY: f32 = 15.0;
pub const FONT_SIZE_BUTTON: f32 = 15.0;
pub const FONT_SIZE_SUBHEADING: f32 = 17.5;
pub const FONT_SIZE_HEADING: f32 = 23.0;
pub const TEXT_STYLE_SUBHEADING: &str = "subheading";

// Khoảng đệm, bo góc và kích thước hộp kiểm dùng chung
pub const SPACE_ITEM_X: f32 = 10.0;
pub const SPACE_ITEM_Y: f32 = 10.0;
pub const BTN_PAD_X: f32 = 14.0;
pub const BTN_PAD_Y: f32 = 8.0;
pub const CARD_MARGIN_I8: i8 = 16;
pub const CANVAS_MARGIN_I8: i8 = 20;
pub const SIDEBAR_MARGIN_I8: i8 = 16;
pub const RADIUS_CARD_U8: u8 = 8;
pub const RADIUS_WIDGET_U8: u8 = 6;
pub const RADIUS_PILL_U8: u8 = 99;
pub const CHECKBOX_ICON_WIDTH: f32 = 20.0;
pub const CHECKBOX_ICON_INNER: f32 = 12.0;
pub const CHECKBOX_ICON_SPACING: f32 = 8.0;
pub const SEARCH_INPUT_WIDTH: f32 = 280.0;
pub const HOST_INPUT_WIDTH: f32 = 220.0;
pub const PORT_SPEC_INPUT_WIDTH: f32 = 320.0;
pub const PORT_SINGLE_INPUT_WIDTH: f32 = 90.0;
pub const TWO_COL_BREAKPOINT: f32 = 760.0;

// Mức phóng đại giao diện mặc định và các lựa chọn nhanh
pub const DEFAULT_UI_SCALE: f32 = 1.15;
pub const MIN_UI_SCALE: f32 = 0.90;
pub const MAX_UI_SCALE: f32 = 1.75;
pub const UI_SCALE_PRESETS: &[(f32, &str)] = &[
    (0.90, "90%"),
    (1.0, "100%"),
    (1.15, "115%"),
    (1.25, "125%"),
    (1.40, "140%"),
    (1.50, "150%"),
];

// Hằng số giới hạn cổng, host, timeout và luồng xử lý
pub const MIN_PORT: u16 = 1;
pub const MAX_PORT: u16 = 65535;
pub const MAX_SCAN_PORTS_COUNT: usize = 10000;
pub const MAX_HOST_LENGTH: usize = 253;
pub const MAX_DNS_LABEL_LENGTH: usize = 63;
pub const MIN_TIMEOUT_MS: u64 = 20;
pub const MAX_TIMEOUT_MS: u64 = 10000;
pub const DEFAULT_TIMEOUT_MS: u64 = 250;
pub const MIN_CONCURRENCY: usize = 1;
pub const MAX_CONCURRENCY: usize = 256;
pub const DEFAULT_CONCURRENCY: usize = 64;
pub const DEFAULT_REFRESH_INTERVAL_SECS: u64 = 5;
pub const MIN_REFRESH_INTERVAL_SECS: u64 = 1;
pub const MAX_REFRESH_INTERVAL_SECS: u64 = 60;
pub const DEFAULT_DARK_MODE: bool = true;
pub const DEFAULT_LISTEN_ONLY: bool = false;

// Giá trị khởi tạo mặc định hiển thị trên các biểu mẫu công cụ
pub const DEFAULT_SCAN_HOST: &str = "127.0.0.1";
pub const DEFAULT_SCAN_PORT_SPEC: &str = "21-22,80,443,3000-3010,5173,5432,6379,8000,8080,8443";
pub const DEFAULT_DNS_HOST: &str = "localhost";
pub const DEFAULT_TCP_PING_PORT: &str = "80";
pub const DEFAULT_JSON_SAMPLE: &str =
    "{\"app\":\"DevUtility Hub\",\"crossPlatform\":true,\"platforms\":[\"Windows\",\"macOS\",\"Linux\"]}";

// Mẫu văn bản gốc và văn bản sửa đổi cho Diff Checker
pub const DEFAULT_DIFF_LEFT_SAMPLE: &str =
    "function calculateTotal(items) {\n    let total = 0;\n    for (let i = 0; i < items.length; i++) {\n        total += items[i].price;\n    }\n    return total;\n}";
pub const DEFAULT_DIFF_RIGHT_SAMPLE: &str =
    "function calculateTotal(items, discount = 0) {\n    const subtotal = items.reduce((sum, item) => sum + item.price, 0);\n    const finalTotal = subtotal * (1 - discount);\n    return Math.max(0, finalTotal);\n}";

// Danh sách PID hệ thống quan trọng được bảo vệ khỏi thao tác Kill
pub const PROTECTED_SYSTEM_PIDS: &[u32] = &[0, 4];

// Tên file và khóa biến môi trường
pub const ENV_LOCAL_FILENAME: &str = ".env.local";
pub const ENV_REFRESH_INTERVAL_KEY: &str = "APP_REFRESH_INTERVAL_SECS";
pub const ENV_TIMEOUT_MS_KEY: &str = "SCANNER_DEFAULT_TIMEOUT_MS";
pub const ENV_CONCURRENCY_KEY: &str = "SCANNER_DEFAULT_CONCURRENCY";
pub const ENV_DARK_MODE_KEY: &str = "APP_DEFAULT_DARK_MODE";
pub const ENV_UI_SCALE_KEY: &str = "APP_DEFAULT_UI_SCALE";
pub const ENV_LISTEN_ONLY_KEY: &str = "APP_DEFAULT_LISTEN_ONLY";

// Nhãn mặc định khi không truy xuất được trạng thái, tiến trình hoặc dịch vụ
pub const UNKNOWN_PROCESS_NAME: &str = "Không xác định";
pub const UNKNOWN_SERVICE_NAME: &str = "Không rõ";
pub const UDP_UNCONN_STATE: &str = "UNCONN";

// Danh sách cổng Developer và dịch vụ phổ biến để quét nhanh và nhận diện
pub const COMMON_DEV_PORTS: &[(u16, &str)] = &[
    (21, "FTP"),
    (22, "SSH"),
    (25, "SMTP"),
    (53, "DNS"),
    (80, "HTTP"),
    (443, "HTTPS"),
    (1433, "SQL Server"),
    (1521, "Oracle DB"),
    (2375, "Docker API"),
    (3000, "Node / React Dev"),
    (3306, "MySQL / MariaDB"),
    (4000, "GraphQL / Dev"),
    (4200, "Angular Dev"),
    (5000, "Flask / Python"),
    (5173, "Vite Dev"),
    (5432, "PostgreSQL"),
    (5672, "RabbitMQ"),
    (6379, "Redis"),
    (8000, "Django / HTTP Alt"),
    (8080, "HTTP Proxy / Tomcat"),
    (8443, "HTTPS Alt"),
    (8888, "Jupyter / Alt"),
    (9000, "PHP-FPM / SonarQube"),
    (9090, "Prometheus"),
    (9200, "Elasticsearch"),
    (27017, "MongoDB"),
];

// Đường dẫn font Unicode hệ thống cho Windows, macOS và Linux để hiển thị tiếng Việt
pub const WINDOWS_FONT_PATHS: &[&str] = &[
    "C:\\Windows\\Fonts\\segoeui.ttf",
    "C:\\Windows\\Fonts\\arial.ttf",
];
pub const WINDOWS_MONO_FONT_PATHS: &[&str] = &[
    "C:\\Windows\\Fonts\\consola.ttf",
    "C:\\Windows\\Fonts\\cour.ttf",
];
pub const MACOS_FONT_PATHS: &[&str] = &[
    "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
    "/System/Library/Fonts/SFNS.ttf",
];
pub const MACOS_MONO_FONT_PATHS: &[&str] = &[
    "/System/Library/Fonts/Menlo.ttc",
    "/System/Library/Fonts/Monaco.ttf",
];
pub const LINUX_FONT_PATHS: &[&str] = &[
    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    "/usr/share/fonts/TTF/DejaVuSans.ttf",
    "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
];
pub const LINUX_MONO_FONT_PATHS: &[&str] = &[
    "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
    "/usr/share/fonts/TTF/DejaVuSansMono.ttf",
];
