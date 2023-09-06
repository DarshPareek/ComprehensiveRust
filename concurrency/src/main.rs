use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });
    //let res = handle.join(); //main does not wait for the other thread but uncommenting this line makes it wait.
    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
    let s = String::from("Hello");

    thread::scope(|scope| {
        // this is a scoped thread which can use values in the current scope
        scope.spawn(|| {
            println!("Length: {}", s.len());
        });
    });
    let (tx, rx) = mpsc::channel();
    tx.send(10).unwrap();
    tx.send(20).unwrap();
    println!("Recieved {:?}", rx.recv());
    println!("Recieved {:?}", rx.recv());

    let (tx, rx) = mpsc::sync_channel(0);
    thread::spawn(move || {
        let tid = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{tid:?} sent a message {i}");
        }
        println!("{tid:?} is now done");
    });
    thread::sleep(Duration::from_millis(10));
    for msg in rx.iter() {
        println!("{msg:?}");
    }
}
