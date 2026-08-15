# Din

Lightweight CLI written in Rust for making periodic HTTP requests to an endpoint.

Din is primarily designed to keep web services awake when they are automatically suspended after periods of inactivity, as commonly happens on free-tier cloud platforms such as Render and similar services.

---

## 🚀 What problem does it solve?

Many cloud providers on free-tier plans may:

- Suspend applications after a period of inactivity.
- Introduce slow cold starts when the next request arrives.
- Take several seconds to respond to the first request after being suspended.

Din sends periodic HTTP requests to:

- Keep a service active.
- Reduce cold-start latency.
- Detect service failures and HTTP errors.
- Provide basic availability and response-time statistics.

---

## ⚙️ What does it do?

Din:

- Sends HTTP GET requests to a configured URL.
- Executes requests at configurable intervals.
- Supports a configurable request timeout.
- Displays request results in real time.
- Tracks basic request statistics.
- Handles `Ctrl+C` for graceful shutdown.

The first request is sent immediately after startup. Subsequent requests are performed after the configured interval.

---

## 📦 Installation

### Option 1 — Build from source

#### Requirements

- Rust and Cargo installed.

Clone the repository:

```bash
git clone https://github.com/davidsandez/din.git
cd din
```

Build the release binary:

```bash
cargo build --release
```

Install globally:

```bash
cargo install --path .
```

This makes `din` available as a global command.

---

### Option 2 — Use the compiled binary

Din compiles into a standalone executable.

After running:

```bash
cargo build --release
```

the executable will be available at:

```text
target/release/din
```

You can copy the binary to another machine, upload it to a server, or install it manually.

For example:

```bash
sudo cp target/release/din /usr/local/bin/
```

---

## ▶️ Usage

### Basic usage

```bash
din https://example.com
```

By default, Din sends a request every 120 seconds.

### With options

```bash
din https://example.com -i 60 -t 5
```

The equivalent long-form command is:

```bash
din https://example.com --interval 60 --timeout 5
```

### Available options

|Option|Description|Default|
|---|---|---|
|Positional `URL`|URL to monitor|Required|
|`-i`, `--interval`|Interval between requests, in seconds|`120`|
|`-t`, `--timeout`|Request timeout, in seconds|`10`|

Both `interval` and `timeout` must be greater than zero.

You can also display the available options with:

```bash
din --help
```

---

## 📊 Example output

When Din starts:

```text
🚀 Starting monitoring of: https://example.com
⏱️  Interval: 60 seconds
⏳ Timeout: 5 seconds
Press Ctrl+C to stop

✅ [2026-02-21 14:32:10] #1 OK - 0.21s
```

When the server responds with a non-successful HTTP status:

```text
⚠️  [2026-02-21 14:33:10] #2 HTTP 500 Internal Server Error - 0.18s
```

When a request fails:

```text
❌ [2026-02-21 14:34:10] #3 ERROR - error sending request - 5.01s
```

When Din receives `Ctrl+C`:

```text
🛑 Signal received. Closing immediately...

📊 Final statistics:
   Total: 15
   Success: 15 (100.0%)
   Errors: 0 (0.0%)
👋 END.
```

---

## 🧠 Typical use cases

- Keep a backend service active on Render.
- Keep an API awake on Railway.
- Reduce cold starts on services that suspend after inactivity.
- Perform basic HTTP availability checks.
- Manually test service stability and response times.

---

## 🔒 Limitations

Din is intentionally simple and is not intended to replace a professional monitoring system.

It currently:

- Does not provide alerts or notifications.
- Does not retry failed requests within the same cycle.
- Does not perform requests in parallel.
- Does not provide persistent monitoring history.
- Only performs HTTP GET requests.

Din is designed to be simple, deterministic, and transparent.

---

## 🛠️ Development

Check the project:

```bash
cargo check
```

Format the code:

```bash
cargo fmt
```

Verify formatting without modifying files:

```bash
cargo fmt --check
```

Run Clippy with warnings treated as errors:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Build the release binary:

```bash
cargo build --release
```

Run the test suite:

```bash
cargo test
```

---

## 📄 License

Din is released under the MIT License.

See the `LICENSE` file for the complete license text.

---

## 📌 Philosophy

Din is built around a few simple principles:

- **Minimal** — focused on one specific task.
- **Lightweight** — distributed as a single Rust binary.
- **Deterministic** — predictable execution and configuration.
- **Transparent** — request results and statistics are displayed directly in the terminal.
- **Easy to distribute** — no runtime infrastructure is required.

Din is intended for developers who need a straightforward way to perform periodic HTTP checks without deploying additional monitoring infrastructure.