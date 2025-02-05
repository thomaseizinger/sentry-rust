use std::io;

mod shared;

#[test]
fn synthetic_exception_should_have_same_name_as_event() {
    let transport = shared::init_sentry(0.0); // This test should work even if we are not sampling transactions.

    foo();

    let data = transport.fetch_and_clear_envelopes();
    assert_eq!(data.len(), 1);

    let event = data.first().expect("should have 1 event");
    let event = match event.items().next().unwrap() {
        sentry::protocol::EnvelopeItem::Event(event) => event,
        unexpected => panic!("Expected event, but got {:#?}", unexpected),
    };

    assert_eq!(event.exception.len(), 2);
    assert_eq!(
        event.exception[0].value,
        Some("Something failed".to_owned())
    );
    assert_eq!(event.exception[1].value, Some("boom!".to_owned()));
    assert_eq!(event.message, Some("boom!".to_owned()));
}

fn foo() {
    let error = io::Error::other("Something failed");

    tracing::error!(error = (&error as &dyn std::error::Error), "boom!");
}
