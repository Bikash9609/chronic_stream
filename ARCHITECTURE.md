Here’s a **visual map** of your Rust engine’s folder structure **plus** the high-level flow of calls/functions (all kept abstract—no actual code). Think of it as your blueprint:

---

```plaintext
my_engine/
├── Cargo.toml             # Package config
├── config.yaml            # (Optional) Default/sample user config
└── src/
    ├── main.rs            # Entry point
    ├── config.rs          # Logic to read & parse config
    ├── config_types.rs    # Structs matching config file
    ├── scheduler.rs       # Polling and task orchestration
    ├── pipeline.rs        # Handles transforms → outputs
    ├── transforms/        # Transformation modules
    │   ├── mod.rs
    │   ├── filter.rs
    │   └── enrich.rs
    ├── outputs/           # Output adapters (DBs, webhooks, etc.)
    │   ├── mod.rs
    │   └── postgres.rs
    └── plugins.rs         # JS/Python plugin execution
```

---

## 🔄 High-Level Call Flow

```plaintext
main.rs
└─► bootstrap():
      ├─► config::parse_config(path)  ──► returns Config
      ├─► plugins::init(config.plugins)
      └─► scheduler::start_all(config.sources, config.transforms, config.outputs)

scheduler.rs (for each source)
└─► schedule_tasks(source_cfg, |event_json| {
      └─► pipeline::process_event(event_json, &config.transforms, &config.outputs)
   })

pipeline.rs
└─► process_event(event, transforms, outputs):
      ├─► for each transform in transforms:
      │     └─► transforms::<type>(event, &rule)
      └─► for each output in outputs:
            └─► outputs::<type>(event, &output_cfg)
```

---

### 📋 Function Prototypes (Conceptual)

* **parse\_config**
  `fn parse_config(path: &str) -> Result<Config, Error>`

* **schedule\_tasks**
  `fn schedule_tasks(src: SourceCfg, handler: impl Fn(Value) + Send + 'static)`

* **process\_event**
  `fn process_event(mut event: Value, transforms: &[TransformCfg], outputs: &[OutputCfg])`

* **transform functions**

  ```text
  fn filter(event: &Value, rule: &FilterRule) -> bool
  fn rename(event: &mut Value, map: &RenameMap)
  fn add_fields(event: &mut Value, fields: &FieldMap)
  ```

* **output functions**

  ```text
  fn send_to_postgres(event: &Value, cfg: &DbCfg) -> Result<(), Error>
  fn send_webhook(event: &Value, cfg: &WebhookCfg) -> Result<(), Error>
  ```

* **plugin executor**
  `fn run_plugin(event: &Value, plugin: &PluginCfg) -> Result<Value, Error>`

---

With this map you can see:

1. **Where** each responsibility lives (config, scheduling, pipeline, transforms, outputs, plugins).
2. **How** data flows from `main` → scheduler → pipeline → transforms → outputs.
3. **Which** abstract functions you’ll need to implement in each module.

This should give you a clear blueprint for coding (or for explaining to your team) without diving into actual Rust code just yet.
