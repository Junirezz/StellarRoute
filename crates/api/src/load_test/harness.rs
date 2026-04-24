use tokio::sync::mpsc;

// other imports

fn some_function() {
    let (tx, _rx) = mpsc::channel::<()>(...);
    // implementation details
}
