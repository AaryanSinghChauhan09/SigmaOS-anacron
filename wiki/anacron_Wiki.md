# SigmaOS-anacron Component Wiki

## Overview
SigmaOS-anacron provides cron daemon and scheduled task support for the SigmaOS operating system. It demonstrates the task scheduling subsystem and cron integration within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic cron initialization.

## Principles
- Single Responsibility
- Scheduling Abstraction
- Fail-Fast on Scheduling Initialization Errors
- Deterministic Scheduling Setup
- Clear Abstraction Over Scheduling APIs
- Composition over Inheritance for scheduling extensions

## Architecture
SigmaOS-anacron follows a scheduling pattern:
- **Scheduler Core**: Provides cron job management, scheduled task execution, and task scheduling
- **Cron Integration**: Supports cron format, cron job definitions, and cron schedule parsing
- **Task Execution**: Supports scheduled task execution, task retry, and task timeout
- **Error Handling**: Fail-fast on invalid scheduling configurations or unsupported scheduler types

## API
The component provides the following public APIs:
- `schedule::cron::parse(expression)` - Parse a cron expression and generate schedule
- `schedule::job::add(job)` - Add a scheduled job with given cron expression
- `schedule::task::execute(task)` - Execute a scheduled task
- `schedule::status::jobs()` - Get scheduled jobs and scheduling status
- Error types for scheduling failure scenarios or unsupported scheduler types

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific scheduling configuration
2. **Act**: Execute the code under test (cron parsing, job addition)
3. **Assert**: Verify the expected scheduling results

Example:
```rust
#[test]
fn test_schedule_cron_aaa() {
    // Arrange
    let expression = "0 * * * *";
    
    // Act
    let result = schedule::cron::parse(expression);
    
    // Assert
    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert!(parsed.is_valid());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Scheduling initialization is best-effort; gracefully degrades on unsupported configurations
- Comprehensive error types for all scheduling failure scenarios
- Unit tests provided for cron parsing and job addition
- Integration tests for full scheduling system with actual cron integration

## TODO
- Add support for additional schedule types (interval, countdown, recurring)
- Expand cron capabilities (cron expressions, time zones, cron validation)
- Add task features (task priorities, task dependencies, task cancellation)
- Create integration tests with actual cron systems and task scheduling
- Implement scheduling persistence and scheduler state management