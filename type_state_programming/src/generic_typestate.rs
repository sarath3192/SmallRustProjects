// Red    -> Green
// Green  -> Yellow
// Yellow -> Red
//
// All    -> Fault ( All = red, yellow, green)
// Fault  -> Red

// This is the state transition of signals we are trying to achieve

use std::marker::PhantomData;

pub trait SignalState{}

pub struct Red;
pub struct Yellow;
pub struct Green;
pub struct Fault;

impl SignalState for Red {}
impl SignalState for Yellow{}
impl SignalState for Green{}
impl SignalState for Fault{}

pub struct TrafficSignal<S: SignalState>{
    _marker: PhantomData<S>,
}

impl <S: SignalState> TrafficSignal<S>{
   
    fn transition() -> TrafficSignal<S>
    {
        TrafficSignal
        {
        _marker: PhantomData,
        }
    }

    pub fn fault1(self) -> TrafficSignal<Fault>{
        
        TrafficSignal::transition()
    }
}

impl TrafficSignal<Red>{
    
    pub fn next(self) -> TrafficSignal<Green>{
       
       TrafficSignal::transition()
    }
}

impl TrafficSignal<Green> {
    
    pub fn next(self) -> TrafficSignal<Yellow> {
        
        TrafficSignal::transition()
    }
}

impl TrafficSignal<Yellow>{
    pub fn next(self) -> TrafficSignal<Red>{
        TrafficSignal::transition()
    }
}

impl TrafficSignal<Fault>{
    pub fn clear_fault(self) -> TrafficSignal<Red>{
        TrafficSignal::transition()
    }
    pub fn initial() -> TrafficSignal<Fault>{
        TrafficSignal::transition()
    }
}

