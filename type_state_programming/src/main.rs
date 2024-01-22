// mod typestate1;

// use std::marker::PhantomData;

mod generic_typestate;
use generic_typestate::*;

fn main() {
    
//  let teststring = "Hello world from sarath";
//  let final_string = Rawtext::new(& teststring).parse().format();
//  println!("{:?}",final_string);


    // TrafficSignal starts in a fault state
    // and must be manually cleared.
    let signal: TrafficSignal<Fault> = TrafficSignal::initial();
    let signal: TrafficSignal<Red> = signal.clear_fault();

    // Distinct `next` functions transition from one
    // state to the next:
    let signal: TrafficSignal<Green> = signal.next();
    let signal: TrafficSignal<Yellow> = signal.next();
    let signal: TrafficSignal<Red> = signal.next();

    // We don't need to use any type annotations. Rust
    // keeps track of the state for us:
    let signal = signal.next(); // green
    let signal = signal.fault1();
    let signal = signal.clear_fault();
    let signal = signal.next(); // yellow
    let signal = signal.fault1();
    let signal = signal.clear_fault();
    let signal = signal.next();
// red

}