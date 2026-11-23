#![allow(non_snake_case)]

use std::fs;
use std::net::TcpListener;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

use dev_utility_hub::config::env_config::AppEnvConfig;
use dev_utility_hub::models::diff_models::DiffOptions;
use dev_utility_hub::models::port_models::ScanProgressMessage;
use dev_utility_hub::services::diff_service::computeTextDiff;
use dev_utility_hub::services::network_service::resolveHostAddresses;
use dev_utility_hub::services::scanner_service::startBackgroundScan;
use dev_utility_hub::utils::input_validator::{
    parsePortInput, parsePortSpecification, validateConcurrency, validateHost, validateKillablePid,
    validateTimeoutMs,
};

#[test]
fn testValidateHostAcceptsValidInputsAndRejectsDangerousInputs() {
    assert_eq!(validateHost("127.0.0.1").unwrap(), "127.0.0.1");
    assert_eq!(validateHost("  localhost  ").unwrap(), "localhost");
    assert_eq!(validateHost("api.example-dev.local").unwrap(), "api.example-dev.local");
    assert_eq!(validateHost("::1").unwrap(), "::1");

    assert!(validateHost("").is_err());
    assert!(validateHost("   ").is_err());
    assert!(validateHost("127.0.0.1; rm -rf /").is_err());
    assert!(validateHost("host|whoami").is_err());
    assert!(validateHost("host&calc").is_err());
    assert!(validateHost("bad..host.com").is_err());
    assert!(validateHost(".leadingdot.com").is_err());
    assert!(validateHost("999.999.999.999").is_err());
    assert!(validateHost("127.0.0.999").is_err());
    assert!(validateHost("a.-b.com").is_err());
}

#[test]
fn testParsePortSpecificationExpandsAndDeduplicatesRanges() {
    let parsedPorts = parsePortSpecification("80, 443, 3000-3002, 80").unwrap();
    assert_eq!(parsedPorts, vec![80, 443, 3000, 3001, 3002]);

    assert_eq!(parsePortInput("8080").unwrap(), 8080);
    assert!(parsePortInput("0").is_err());
    assert!(parsePortInput("70000").is_err());
    assert!(parsePortSpecification("3005-3000").is_err());
    assert!(parsePortSpecification("80,,443").is_err());
    assert!(parsePortSpecification("1-65535").is_err());
}

#[test]
fn testValidateLimitsAndProtectedPidGuards() {
    assert!(validateTimeoutMs(250).is_ok());
    assert!(validateTimeoutMs(5).is_err());
    assert!(validateTimeoutMs(99999).is_err());

    assert!(validateConcurrency(32).is_ok());
    assert!(validateConcurrency(0).is_err());
    assert!(validateConcurrency(1000).is_err());

    let currentPid = std::process::id();
    assert!(validateKillablePid(0, currentPid).is_err());
    assert!(validateKillablePid(4, currentPid).is_err());
    assert!(validateKillablePid(currentPid, currentPid).is_err());
    assert_eq!(validateKillablePid(99999, currentPid).unwrap(), 99999);
}

#[test]
fn testResolveIpv6LoopbackDoesNotFailOnSocketFormatting() {
    let result = resolveHostAddresses("::1").expect("Phải phân giải được địa chỉ IPv6 ::1");
    assert!(result.addresses.iter().any(|addr| addr == "::1"));
}

#[test]
fn testSettingsSaveAndLoadPreservesOtherEnvVariables() {
    let tempPath = std::env::temp_dir().join(format!(
        "dev_utility_hub_settings_test_{}.env",
        std::process::id()
    ));
    let initialContent = "# Comment giữ lại\nUSER_CUSTOM_SECRET=preserved_123\n";
    fs::write(&tempPath, initialContent).expect("Phải ghi được file env tạm");

    let customConfig = AppEnvConfig {
        refreshIntervalSecs: 12,
        defaultTimeoutMs: 750,
        defaultConcurrency: 128,
        defaultDarkMode: false,
        defaultUiScale: 1.25,
        defaultListenOnly: true,
    };

    customConfig
        .saveToFile(&tempPath)
        .expect("Lưu cấu hình sang file tạm phải thành công");

    let rawSaved = fs::read_to_string(&tempPath).unwrap();
    assert!(rawSaved.contains("# Comment giữ lại"));
    assert!(rawSaved.contains("USER_CUSTOM_SECRET=preserved_123"));

    let reloaded = AppEnvConfig::loadFromFile(&tempPath);
    let _ = fs::remove_file(&tempPath);

    assert!(
        reloaded.matchesConfig(&customConfig),
        "Cấu hình đọc lại phải khớp toàn bộ 6 trường đã lưu"
    );
}

#[test]
fn testBackgroundPortScannerDetectsOpenLoopbackPort() {
    // Mở một cổng TCP tạm thời trên loopback để kiểm chứng chức năng quét
    let listener = TcpListener::bind("127.0.0.1:0").expect("Phải bind được cổng test tạm thời");
    let openPort = listener.local_addr().unwrap().port();

    let cancelFlag = Arc::new(AtomicBool::new(false));
    let receiver = startBackgroundScan(
        "127.0.0.1".to_string(),
        vec![openPort],
        500,
        1,
        cancelFlag,
    );

    let mut foundOpen = false;
    let mut completed = false;

    while let Ok(msg) = receiver.recv_timeout(Duration::from_secs(5)) {
        match msg {
            ScanProgressMessage::PortChecked(item) => {
                if item.port == openPort && item.isOpen {
                    foundOpen = true;
                }
            }
            ScanProgressMessage::Completed => {
                completed = true;
                break;
            }
            ScanProgressMessage::Cancelled | ScanProgressMessage::Failed(_) => break,
        }
    }

    drop(listener);
    assert!(completed, "Quá trình quét cổng thử nghiệm phải hoàn tất");
    assert!(foundOpen, "Port Scanner phải phát hiện đúng cổng đang mở");
}

#[test]
fn testDiffServiceDetectsAdditionsDeletionsAndUnchangedLines() {
    let oldCode = "line1\nline2\nline3";
    let newCode = "line1\nline2_modified\nline3\nline4_added";
    let opts = DiffOptions::default();

    let result = computeTextDiff(oldCode, newCode, &opts);
    assert_eq!(result.summary.unchanged, 2, "line1 và line3 phải giữ nguyên");
    assert_eq!(result.summary.deletions, 1, "line2 phải bị xóa");
    assert_eq!(result.summary.additions, 2, "line2_modified và line4_added phải được thêm");

    // Kiểm tra hàng Split View được dựng hợp lý
    assert!(!result.splitRows.is_empty(), "Phải sinh ra các hàng split");
}

#[test]
fn testDiffServiceHandlesWhitespaceAndCaseInsensitiveOptions() {
    let left = "  HELLO World  \nLine 2";
    let right = "hello world\nLine 2";

    // Mặc định phân biệt hoa/thường và khoảng trắng
    let strictOpts = DiffOptions::default();
    let strictResult = computeTextDiff(left, right, &strictOpts);
    assert_eq!(strictResult.summary.unchanged, 1);
    assert_eq!(strictResult.summary.deletions, 1);
    assert_eq!(strictResult.summary.additions, 1);

    // Bật bỏ qua khoảng trắng và không phân biệt hoa thường
    let relaxedOpts = DiffOptions {
        ignoreWhitespace: true,
        caseSensitive: false,
    };
    let relaxedResult = computeTextDiff(left, right, &relaxedOpts);
    assert_eq!(relaxedResult.summary.unchanged, 2, "Cả hai dòng đều phải tính là khớp khi bỏ qua case & whitespace");
    assert_eq!(relaxedResult.summary.additions, 0);
    assert_eq!(relaxedResult.summary.deletions, 0);
}
