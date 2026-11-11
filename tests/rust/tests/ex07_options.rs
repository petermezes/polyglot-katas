use std::time::Duration;

use under_test::ex07_options::*;

#[test]
fn defaults() {
    let c = ClientBuilder::new("https://api.example.no")
        .build()
        .expect("unexpected error");
    assert_eq!(c.base_url, "https://api.example.no");
    assert_eq!(c.timeout, Duration::from_secs(30));
    assert_eq!(c.retries, 3);
    assert_eq!(c.user_agent, "exercises/1.0");
    assert!(c.headers.is_empty());
}

#[test]
fn setters_are_applied() {
    let c = ClientBuilder::new("https://api.example.no")
        .timeout(Duration::from_secs(5))
        .retries(0)
        .header("Accept", "application/json")
        .header("X-Tunnel", "Løvstakk")
        .user_agent("vegvokter-test")
        .build()
        .expect("unexpected error");
    assert_eq!(c.timeout, Duration::from_secs(5));
    assert_eq!(c.retries, 0);
    assert_eq!(c.user_agent, "vegvokter-test");
    assert_eq!(c.headers.len(), 2);
    assert_eq!(c.headers["X-Tunnel"], "Løvstakk");
}

#[test]
fn later_calls_win() {
    let c = ClientBuilder::new("https://x")
        .retries(2)
        .retries(4)
        .build()
        .unwrap();
    assert_eq!(c.retries, 4, "setters should be applied in order");
}

#[test]
fn invalid_values() {
    let insecure = ClientBuilder::new("http://api.example.no")
        .build()
        .unwrap_err();
    assert_eq!(
        insecure,
        ConfigError::InsecureUrl {
            url: "http://api.example.no".to_string()
        }
    );
    assert_eq!(
        insecure.to_string(),
        "URL must start with https://: http://api.example.no"
    );

    let timeout = ClientBuilder::new("https://x")
        .timeout(Duration::ZERO)
        .build()
        .unwrap_err();
    assert_eq!(
        timeout,
        ConfigError::InvalidTimeout {
            timeout: Duration::ZERO
        }
    );
    assert_eq!(timeout.to_string(), "timeout 0ns: must be positive");

    let retries = ClientBuilder::new("https://x")
        .retries(11)
        .build()
        .unwrap_err();
    assert_eq!(retries, ConfigError::InvalidRetries { retries: 11 });
    assert_eq!(retries.to_string(), "retries 11: allowed 0 to 10");
    // No retries(-1) case like in Go: u32 makes it a compile error.
}

#[test]
fn builder_can_be_reused_through_clone() {
    // The builder is consumed by build(). To build two variants, clone it first.
    let base = ClientBuilder::new("https://x").header("A", "1");
    let a = base.clone().retries(1).build().unwrap();
    let b = base.header("B", "2").build().unwrap();
    assert_eq!(a.retries, 1);
    assert_eq!(a.headers.len(), 1, "a must not see b's header");
    assert_eq!(b.retries, 3);
    assert_eq!(b.headers.len(), 2);
}

#[test]
fn clone_is_deep() {
    let orig = ClientBuilder::new("https://x")
        .header("A", "1")
        .build()
        .unwrap();
    let mut copy = orig.clone();
    copy.headers.insert("B".to_string(), "2".to_string());
    copy.retries = 9;
    assert_eq!(
        orig.headers.len(),
        1,
        "the clone shares headers with the original"
    );
    assert_eq!(orig.retries, 3, "changing the copy affected the original");
}

#[test]
fn slower_returns_a_modified_copy() {
    let orig = ClientBuilder::new("https://x")
        .header("A", "1")
        .build()
        .unwrap();
    let slow = orig.slower(2.0);
    assert_eq!(slow.timeout, Duration::from_secs(60));
    assert_eq!(slow.headers, orig.headers);
    assert_eq!(
        orig.timeout,
        Duration::from_secs(30),
        "the original must stay unchanged"
    );
}
