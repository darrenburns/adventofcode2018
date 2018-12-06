enum Action {
    StartShift,
    FallAsleep,
    WakeUp,
}

struct LogEntry {
    year: u8,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    guard_id: u8,
    action: Action,
}

