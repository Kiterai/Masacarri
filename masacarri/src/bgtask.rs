use std::env;

use actix::{Addr, SyncArbiter, SyncContext};

#[derive(Default)]
pub struct BgActor;

impl actix::Actor for BgActor {
    type Context = actix::SyncContext<Self>;

    fn started(&mut self, _: &mut SyncContext<Self>) {}
}

pub type BgTaskManager = Addr<BgActor>;

pub fn make_bgtask_manager() -> Addr<BgActor> {
    let bgtask_threadnum = env::var("BGTASK_THREADNUM")
        .unwrap_or("16".to_string())
        .parse::<usize>()
        .expect("BGTASK_THREADNUM is invalid");
    SyncArbiter::start(bgtask_threadnum, move || BgActor::default())
}
