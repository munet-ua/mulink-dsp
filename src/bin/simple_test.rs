use log::trace;
use mulink_dsp::{core::{signal::{FromVec, Signal}, stream::AudioStream}, logging::{init_logging, init_tracing}};

fn main() -> anyhow::Result<()> {
    init_tracing();
    trace!("INST stream");
    let stream = AudioStream::<f32>::new();

    trace!("INST subs");
    let sub1 = stream.get_subscriber();
    let sub2 = stream.get_subscriber();

    trace!("lock1");
    {
        let tx = stream.lock()?;
        tx.send(Signal::from_vec(192000.0, vec![0.0; 128]))?;
    }

    trace!("recv1");
    let msg1 = sub1.recv()?;
    let msg2 = sub2.recv()?;

    trace!("lock2");
    {
        let tx = stream.lock()?;
        tx.send(Signal::from_vec(192000.0, vec![0.0; 128]))?;
    }

    trace!("recv2");

    let msg1 = sub1.recv()?;
    let msg2 = sub2.recv()?;


    Ok(())
}