# ChronicStream

> **ChronicStream**: Uninterrupted, low-latency data pipelines.

ChronicStream is an open-source, Rust-native engine for ingesting live API data (webhooks/events), applying filters and transformations, and routing it to databases, webhooks, or message queues—**all without requiring users to write Rust code**. It’s designed for ultra-low latency, high throughput, and a simple, declarative configuration.

---

## 📋 Table of Contents

1. [Features](#features)
2. [Why ChronicStream?](#why-chronicstream)
3. [Getting Started](#getting-started)

   * [Prerequisites](#prerequisites)
   * [Installation](#installation)
4. [Configuration](#configuration)

   * [Sample YAML Template](#sample-yaml-template)
5. [Usage Examples](#usage-examples)
6. [Roadmap & Future Scopes](#roadmap--future-scopes)
7. [Release Checklist](#release-checklist)
8. [Contribution Guide](#contribution-guide)

   * [Code of Conduct](#code-of-conduct)
   * [How to Contribute](#how-to-contribute)
   * [Bug Reports & Feature Requests](#bug-reports--feature-requests)
   * [Pull Request Process](#pull-request-process)
9. [License](#license)
10. [Acknowledgments](#acknowledgments)

---

## 🏷 Features

* **Rust-Native Core**
  High-performance, memory-safe, concurrent event engine built in Rust.

* **Declarative Configuration**
  Define data sources (HTTP APIs, webhooks), transformations, and outputs in a YAML/JSON/TOML file—no Rust coding required.

* **Plugin Support (JS/Python)**
  Drop in custom logic written in JavaScript or Python; ChronicStream embeds a sandboxed runtime to execute your code on each event.

* **Multiple Output Adapters**
  · PostgreSQL
  · Kafka
  · Webhooks (HTTP POST)
  · Local file (JSONL)
  · …and more via community-contributed adapters

* **Robust Error Handling & Retry**
  · Exponential backoff on API failures
  · Dead-letter support for unprocessable events
  · Configurable “log and continue” vs. “fail pipeline” options

* **Monitoring & Metrics (Future)**
  · Built-in stats (throughput, errors, latencies)
  · Prometheus ↔ ChronicStream exporter (planned)

---

## ❓ Why ChronicStream?

Many existing ETL and streaming solutions focus on batch processing or require heavyweight infrastructures (e.g., Kafka clusters, Spark clusters).
ChronicStream solves the gap for **lightweight, embeddable, real-time pipelines** that:

* Run in a single binary, with zero external dependencies.
* Require minimal operational overhead—no cluster provisioning.
* Expose a simple configuration DSL to non-Rust developers.
* Deliver near-C-level performance via Rust’s zero-cost abstractions and async runtime.

---

## 🚀 Getting Started

### Prerequisites

1. **Rust toolchain** (with Cargo) installed:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **PostgreSQL (optional)** if you plan to use the Postgres output adapter.

3. **Kafka (optional)** if you plan to use the Kafka output adapter.

> ChronicStream itself is a single Rust binary—no additional services are mandatory.

---

### Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/your-org/chronicstream.git
   cd chronicstream
   ```

2. **Build the Binary**

   ```bash
   cargo build --release
   ```

3. **Install to \$PATH** (optional)

   ```bash
   cp target/release/chronicstream /usr/local/bin/
   chmod +x /usr/local/bin/chronicstream
   ```

4. **Verify**

   ```bash
   chronicstream --help
   ```

   You should see usage instructions and available subcommands.

---

## ⚙ Configuration

ChronicStream is driven by a single configuration file (YAML, JSON, or TOML). Below is a **comprehensive YAML template** (with comments) to initialize your pipelines.

### Sample YAML Template

```yaml
# ┌───────────────────────────────────────────────────────────────────────────────
# │ ChronicStream Pipeline Configuration
# └───────────────────────────────────────────────────────────────────────────────

# === 1. DATA SOURCES ===
sources:
  - name: "orders"                                # string: unique identifier for this source
    type: "api"                                   # enum: "api" | "webhook" | "event_queue"
    url: "https://api.example.com/orders"         # string: endpoint to fetch or listen
    method: "GET"                                 # string: HTTP method for API/webhook
    headers:                                      # map<string,string>: optional HTTP headers
      Authorization: "Bearer {{env:API_TOKEN}}"
    query_params:                                 # map<string,string>: optional query params
      status: "completed"
    polling_interval: 30                          # int: seconds between API polls
    pagination:                                   # object: pagination settings (optional)
      type: "cursor"                              # enum: "cursor" | "offset" | "none"
      cursor_field: "next_page_token"             # string: JSON field name for next cursor
      max_pages: 10                               # int: maximum pages to fetch per run

  - name: "stripe_webhook"
    type: "webhook"                               # for push-based events
    url: "https://my-engine.local/webhook"        # where ChronicStream listens
    method: "POST"                                # HTTP method to receive events
    secret: "{{env:WEBHOOK_SECRET}}"              # string: secret for signature verification

# === 2. TRANSFORMATION PIPELINE ===
transforms:
  - filter:                                       # object: drop or pass events based on condition
      field: "status"                             # string: JSON field to check
      operator: "equals"                          # enum: "equals" | "not_equals" | "contains" | "gt" | "lt"
      value: "completed"                          # any: value to compare against

  - rename:                                       # object: rename JSON fields
      from: "user_id"                             # string: original field name
      to: "customer"                              # string: new field name

  - add_fields:                                   # object: inject static or dynamic fields
      source: "api"                               # string: static value
      fetched_at: "{{timestamp}}"                 # string: will be replaced with the ISO timestamp

  - script:                                       # object: custom JS/Python logic (optional)
      language: "python"                          # enum: "python" | "javascript"
      path: "./plugins/order_transform.py"        # string: path to user-provided script

# === 3. OUTPUTS ===
outputs:
  - type: "postgres"                              # enum: "postgres" | "kafka" | "webhook" | "file"
    table: "processed_orders"                     # string: database table name
    db_url: "{{env:POSTGRES_URL}}"                # string: PostgreSQL connection URL

  - type: "kafka"
    topic: "orders-stream"                        # string: Kafka topic
    brokers:                                      # list<string>: Kafka broker addresses
      - "kafka1:9092"
      - "kafka2:9092"

  - type: "webhook"
    url: "https://analytics.example.com/ingest"   # string: target webhook URL
    method: "POST"                                # string: HTTP method

  - type: "file"
    path: "/var/logs/orders.jsonl"                # string: filesystem path (JSONL format)
    rotate_every_mb: 100                          # int: rotate file when size exceeds MB

# === 4. GLOBAL SETTINGS ===
retry_policy:                                    # object: retries on transient errors
  max_retries: 5                                 # int: how many times to retry
  backoff_base_ms: 200                           # int: base backoff in milliseconds
  backoff_multiplier: 2.0                        # float: exponential multiplier

error_handling:                                  # object: behavior on transform or output errors
  on_error: "log_and_continue"                   # enum: "fail_pipeline" | "log_and_continue" | "dead_letter"
  dead_letter_path: "/var/dead_letters/{{source}}.jsonl"  # string: where to write failed events

logging:
  level: "info"                                  # enum: "debug" | "info" | "warn" | "error"
  format: "json"                                 # enum: "json" | "text"
```

---

## 🔧 Usage Examples

1. **Start ChronicStream with a Config File**

   ```bash
   chronicstream --config ./config.yaml
   ```

2. **Validate Configuration Only**

   ```bash
   chronicstream validate ./config.yaml
   ```

3. **Run in Dry-Run Mode (No Outputs Executed)**

   ```bash
   chronicstream --config ./config.yaml --dry-run
   ```

4. **Verbose Logging for Debugging**

   ```bash
   chronicstream --config ./config.yaml --log-level debug
   ```

> **Note:** Replace `chronicstream` above with the path to your built binary if you didn’t install to `$PATH`.

---

## 🚀 Roadmap & Future Scopes

Below are planned enhancements and ideas for future versions. Feel free to submit issues or PRs if you’d like to help with any of them!

1. **Built-in Monitoring & Metrics**

   * Expose Prometheus metrics (events/sec, error rates, latencies).
   * Web UI dashboard for real-time stats.

2. **Additional Output Adapters**

   * AWS Kinesis, Google Pub/Sub, Azure Event Hubs.
   * Elasticsearch, InfluxDB, and other time-series / search stores.

3. **Data Schema Validation**

   * JSON Schema support to validate incoming events before processing.
   * Automatic rejection or dead-letter routing on schema mismatch.

4. **Graphical UI Builder**

   * Drag-and-drop pipeline designer with live preview.
   * Integrated config editor with syntax highlighting.

5. **Cloud-Native Deployments**

   * Official Docker image, Helm charts for Kubernetes.
   * Managed SaaS offering (ChronicStream Cloud).

6. **Enhanced Plugin Sandboxing**

   * Resource quotas (CPU/memory/time) for JS/Python snippets.
   * Secure containerization for untrusted code.

7. **High-Availability & Clustering**

   * Leader election and distributed task assignment.
   * Horizontal scaling for massive throughput.

8. **Advanced Authentication/Authorization**

   * OAuth2, OpenID Connect support for API sources.
   * Role-based access control (RBAC) within ChronicStream management API.

---

## ✅ Release Checklist

Before tagging a new release (e.g., `v1.0.0`), ensure the following:

* [ ] **Documentation**

  * README updated with new features.
  * CHANGELOG.md drafted with bullet points for major changes.
  * Example configuration templates updated.

* [ ] **Code Quality**

  * All new code passes `cargo fmt` and `cargo clippy`.
  * Unit tests cover new modules/functionality.

* [ ] **Integration Tests**

  * Test real-world end-to-end flow: source → transforms → outputs.
  * Validate retry/backoff and dead-letter logic.

* [ ] **Versioning & Compatibility**

  * Bump version in `Cargo.toml`.
  * Verify backward compatibility or document breaking changes.

* [ ] **Release Artifacts**

  * Build binaries for major platforms (Linux, macOS, Windows).
  * Publish to GitHub Releases or crate registry.

* [ ] **Community Announcement**

  * Blog post or tweet summarizing release highlights.
  * Update website/landing page (if any).

---

## 🤝 Contribution Guide

### Code of Conduct

Please read and follow our [Code of Conduct](./CODE_OF_CONDUCT.md) before contributing. We expect all community members to treat each other with respect and courtesy.

### How to Contribute

1. **Fork the Repository**
   Click “Fork” at the top of the GitHub page and clone your fork:

   ```bash
   git clone https://github.com/your-username/chronicstream.git
   cd chronicstream
   ```

2. **Create a Branch**
   Start a topic branch for your changes:

   ```bash
   git checkout -b feature/my-new-adapter
   ```

3. **Make Your Changes**

   * Follow existing code patterns.
   * Update documentation and tests alongside code.

4. **Run Tests & Linters**

   ```bash
   cargo fmt -- --check
   cargo clippy -- -D warnings
   cargo test
   ```

5. **Commit & Push**

   ```bash
   git add .
   git commit -m "feat: add MyNewAdapter for InfluxDB output"
   git push origin feature/my-new-adapter
   ```

6. **Open a Pull Request**

   * Target branch: `main`
   * Describe what your change does and why it’s needed.
   * Reference any related issues.

### Bug Reports & Feature Requests

* **Search existing issues** to avoid duplicates.
* **Open an issue** using the provided templates:

  * **Bug Report**: Steps to reproduce, expected vs. actual behavior, environment details.
  * **Feature Request**: Describe the use case, proposed design, and any alternatives considered.

### Pull Request Process

1. **Review**

   * Project maintainers will review and request changes if needed.
   * Discussion and feedback happen here—iteratively refine until ready.

2. **CI/CD**

   * Automated tests and linters run on every PR.
   * Ensure all checks pass before merge.

3. **Merge**

   * Once approved and all checks pass, a maintainer merges your PR.
   * Celebrate! 🎉

4. **Post-Merge**

   * A release is created on the next milestone.
   * Documentation and CHANGELOG are updated accordingly.

---

## 📜 License

This project is licensed under the **Apache 2.0 License**. See [LICENSE](./LICENSE) for details.

---

## 🙏 Acknowledgments

* Inspired by [Vector](https://vector.dev), [Logstash](https://www.elastic.co/logstash), and [Temporal](https://temporal.io).
* Thanks to all early contributors and users for feedback and bug reports.
* The Rust community for providing excellent crates like `tokio`, `serde`, and `sqlx`.
