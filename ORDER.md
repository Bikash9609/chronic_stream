Below is a suggested ordering of the first ten functions to implement, in rough dependency order. The goal is to bootstrap the core engine step by step—starting from configuration loading and ending with a concrete output adapter. Each entry includes the function signature (conceptual) and a very brief description.

1. **`fn parse_config(path: &str) -> Result<Config, Error>`**
   - Load the user’s YAML/JSON/TOML file from `path`.
   - Deserialize into a `Config` struct and perform basic structure validation.

2. **`fn validate_config(config: &Config) -> Result<(), Error>`**
   - Check that all required fields (sources, transforms, outputs) are present and well‐formed.
   - Validate enums (e.g., `type == "api" | "webhook"`), intervals > 0, etc.

3. **`fn init_plugins(plugins: &[PluginCfg]) -> Result<(), Error>`**
   - Initialize any plugin runtimes (e.g., spin up a JS sandbox or Python interpreter).
   - Load user‐provided scripts into memory, compile/prewarm as needed.

4. **`fn start_scheduler(sources: &[SourceCfg], transforms: &[TransformCfg], outputs: &[OutputCfg])`**
   - For each entry in `sources`, call `schedule_tasks(source, handler_fn)`.
   - Wire `handler_fn` to invoke `process_event`.

5. **`fn schedule_tasks(src: &SourceCfg, handler: impl Fn(Value) + Send + 'static)`**
   - Spawn a Tokio timer (`tokio::spawn`) that, on each tick, fetches/pulls data from the API or listens for webhooks.
   - On each incoming JSON batch or event, call `handler(event_json)`.

6. **`fn process_event(mut event: Value, transforms: &[TransformCfg], outputs: &[OutputCfg]) -> Result<(), Error>`**
   - Iterate over `transforms` in order, applying each transform (e.g. `filter`, `rename`, `add_fields`, `run_plugin`).
   - If the event survives all filters, iterate through `outputs` and call the appropriate send function for each.

7. **`fn filter(event: &Value, rule: &FilterRule) -> bool`**
   - Return `true` if `event[rule.field]` meets `rule.operator` vs. `rule.value`.
   - `false` means “drop this event” and stop further processing.

8. **`fn rename(event: &mut Value, map: &RenameMap)`**
   - Given a map of `{ from_field: to_field }`, rename keys inside the mutable `event` object in‐place.

9. **`fn add_fields(event: &mut Value, fields: &FieldMap)`**
   - Insert or overwrite static/dynamic fields (e.g. `"fetched_at": "{{timestamp}}"`) into the JSON object.

10. **`fn send_to_postgres(event: &Value, cfg: &DbCfg) -> Result<(), Error>`**
    - Take the final JSON event, serialize it into a SQL `INSERT` or `UPSERT` command (via `sqlx` or similar), and push it into the configured Postgres table.

---

### Notes on Ordering & Dependencies

- **`parse_config` → `validate_config`**: Always load and immediately validate before doing anything else.
- **`init_plugins`** should run right after config validation, so that any user‐provided scripts are available when `process_event` needs to invoke them.
- **`start_scheduler`** and **`schedule_tasks`** come next: once your config is loaded and your plugins are ready, you can start pulling or listening for API data.
- **`process_event`** is the central dispatcher: it must be written before any transform or output functions can be tested.
- **`filter`, `rename`, `add_fields`** are “core” transforms—implement these first so that you can verify basic pipeline logic.
- Finally, **`send_to_postgres`** (and any other output like `send_webhook`, `send_to_kafka`, etc.) should come once you can prove that transformed data is correctly flowing through your pipeline.

You can add additional transform/output functions (e.g., `run_plugin`, `send_webhook`, `send_to_kafka`) once this initial core is working.
