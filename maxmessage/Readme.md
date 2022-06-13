# How to use it? 

```rust
let mock_messenger = MockMessendger::new();

let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
limit_tracker.set_value(80);

assert_eq!(mock_messenger.send_messages.borrow().len(), 1);
```