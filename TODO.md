# TODO

## Critical correctness

- [ ] Stop the booking workflow immediately when a required login element or prerequisite is missing.
- [ ] Track whether a time slot was actually selected before attempting confirmation.
- [ ] Correct the availability handling in `src/main.rs`; the `if` and `else` branches currently do the same thing.
- [ ] Detect unavailable slots reliably, including avoiding ambiguous substring checks such as `contains("disponible")`.
- [ ] Do not confirm a booking when both court/time-slot attempts failed.
- [ ] Verify that the booking succeeded after clicking the confirmation button by waiting for a success message, booking ID, URL change, or equivalent page state.

## Error handling and cleanup

- [ ] Replace Telegram notification `unwrap()` calls with non-panicking error handling.
- [ ] Create a shared notification helper that sends the message and screenshot independently.
- [ ] Guarantee `driver.quit()` runs on every exit path, including errors returned by `?`.
- [ ] Add structured application errors instead of relying on `expect()` and ad hoc `WebDriverResult` values.
- [ ] Include the original Selenium error details in failure notifications or logs where useful.

## Browser automation reliability

- [ ] Replace the fixed two-second login delay with an explicit wait for a post-login element, URL, or authenticated page marker.
- [ ] Replace brittle absolute XPath selectors with stable IDs, relative selectors, or `data-*` attributes.
- [ ] Centralize all selectors as constants so website changes are easier to maintain.
- [ ] Validate the configured `HOUR` before interpolating it into an XPath expression.
- [ ] Avoid interpolating raw configuration into XPath without escaping or validating it.
- [ ] Replace fixed sleeps after clicks with explicit waits for the expected page state.
- [ ] Confirm that the WebDriver session starts with the expected browser capabilities in both local and containerized environments.
- [ ] Decide whether headless mode should be enabled for Docker/Kubernetes and configure it explicitly.

## Date and scheduling

- [ ] Calculate the booking date explicitly in the `Europe/Madrid` timezone rather than relying on the container's local timezone.
- [ ] Configure the container timezone or use `chrono-tz` for timezone-safe date calculations.
- [ ] Make the booking date offset configurable instead of hard-coding two days ahead.
- [ ] Extract date calculation into a pure function that can be unit tested.
- [ ] Confirm that the date format expected by the website remains `%d/%m/%Y`.

## Async and runtime behavior

- [ ] Replace `std::thread::sleep` at the end of `main` with `tokio::time::sleep`.
- [ ] Review all fixed sleeps and replace them with event-based waits where possible.
- [ ] Ensure browser cleanup and Telegram notifications do not block the Tokio runtime unnecessarily.

## Configuration and security

- [ ] Parse all environment variables once into a validated `Config` struct.
- [ ] Improve configuration error messages without exposing credentials or tokens.
- [ ] Treat `TICKETING_URL` as potentially sensitive and document how it should be refreshed or protected.
- [ ] Avoid sending screenshots containing personal or account information unless necessary.
- [ ] Restrict and validate the Telegram chat destination used for notifications.
- [ ] Document the required external Selenium/WebDriver service and browser version.

## Refactoring and testing

- [ ] Split `src/main.rs` into focused modules or functions for configuration, login, date selection, slot selection, confirmation, and notifications.
- [ ] Rename variables for clarity, including `ticketingurl` to `ticketing_url` and `calles` to a meaningful court/slot name.
- [ ] Remove the unnecessary `use chrono;` import and use focused imports.
- [ ] Add unit tests for configuration parsing and validation.
- [ ] Add unit tests for target-date calculation and timezone behavior.
- [ ] Add unit tests for availability-message interpretation.
- [ ] Add tests for the slot-selection state machine without requiring a live browser.
- [ ] Add an integration or smoke-test strategy for the Selenium workflow.
- [ ] Add formatting and lint checks such as `cargo fmt --check` and `cargo clippy -- -D warnings` to CI.
- [ ] Add a Rust test job to GitHub Actions in addition to the Docker build check.
