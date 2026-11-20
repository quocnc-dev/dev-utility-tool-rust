# DevUtility Hub 🚀

Bộ công cụ lập trình viên đa nền tảng (**Windows, macOS, Linux**) được viết thuần bằng **Rust** và **eframe / egui**, đóng gói thành một file nhị phân duy nhất (Single Binary), khởi động tức thì và tiết kiệm tài nguyên.

---

## 🌟 Tính năng chính

1. **Port Manager (Quản lý Port & Tiến trình)**:
   - Liệt kê toàn bộ socket TCP/UDP đang hoạt động qua native system API (`netstat2`).
   - Lọc nhanh theo số Port, tên tiến trình hoặc PID; chuyển đổi xem chỉ các cổng `LISTEN`.
   - **Kill Process an toàn**: Dừng tiến trình chiếm port với hộp thoại xác nhận an toàn, bảo vệ PID hệ thống (0, 4).

2. **Port Scanner (Quét cổng đa luồng)**:
   - Quét cổng bất đồng bộ nền tảng (background threads), không gây đơ giao diện.
   - Hỗ trợ nhập dải cổng linh hoạt (ví dụ: `80,443,3000-3010,8080`) hoặc chọn danh sách Common Dev Ports.
   - Tùy chỉnh Timeout (ms) và số luồng đồng thời (Concurrency).

3. **Network Inspector (Kiểm tra mạng)**:
   - **DNS Lookup**: Phân giải domain ra danh sách địa chỉ IP (IPv4 & IPv6).
   - **TCP Ping**: Kiểm tra khả năng kết nối tới `host:port` và đo độ trễ mạng chính xác theo mili-giây (hỗ trợ cả IPv6 `::1`).

4. **Text Formatter (Định dạng & Chuyển đổi mã)**:
   - **JSON Formatter & Validator**: Định dạng JSON đẹp mắt (Prettify) hoặc nén gọn (Minify), kiểm tra lỗi cú pháp chi tiết.
   - **Base64 Encoder/Decoder**: Mã hóa và giải mã chuỗi văn bản Base64 tiêu chuẩn.
   - **URL Encoder/Decoder**: Mã hóa ký tự đặc biệt cho tham số URL.

5. **Cài đặt hệ thống (Settings)**:
   - Chuyển đổi giao diện **Chế độ Tối (Dark mode)** / **Chế độ Sáng (Light mode)**.
   - Phóng to/thu nhỏ tỷ lệ giao diện mượt mà (**UI Scale 90% – 175%**).
   - Tùy chỉnh tham số mặc định và lưu cấu hình bền vững vào `.env.local`.

---

## 🛠️ Công nghệ sử dụng

- **Ngôn ngữ**: [Rust](https://www.rust-lang.org/) (2021 Edition)
- **GUI Framework**: [eframe / egui 0.37](https://github.com/emilk/egui) + `egui_extras`
- **Mạng & Socket**: `netstat2`, `sysinfo`
- **Cấu hình & Tiện ích**: `dotenvy`, `serde_json`, `base64`, `urlencoding`

---

## 💻 Hướng dẫn chạy & Biên dịch

### 1. Trên Windows
```powershell
# Chạy trực tiếp ở chế độ phát triển
cargo run

# Biên dịch bản tối ưu hóa Release (file nằm tại target/release/dev_utility_hub.exe)
cargo build --release
```

### 2. Trên macOS
```bash
# 1. Cài đặt môi trường (nếu chưa có)
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Chạy ứng dụng
cargo run --release

# 3. (Tùy chọn) Đóng gói thành macOS App (.app)
cargo install cargo-bundle
cargo bundle --release
```

### 3. Trên Linux (Ubuntu/Debian)
```bash
# Cài đặt thư viện đồ họa cần thiết cho egui
sudo apt-get update
sudo apt-get install -y libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev libgtk-3-dev

# Biên dịch và chạy
cargo run --release
```

---

## 🧪 Kiểm thử (Testing)

```bash
cargo test
```
Toàn bộ bài kiểm tra xác thực đầu vào, IPv6 resolution, socket loopback scanner và cấu hình `.env.local` đều được kiểm chứng tự động.

---

## 📄 Bản quyền
Phát triển cho cộng đồng lập trình viên. Tự do sử dụng và mở rộng.
