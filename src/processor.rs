// Copyright (C) 2026 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::ecs::error::{BevyError, Result};
use std::{
    any::Any,
    sync::mpsc::{Receiver, Sender, channel},
    thread::{self, JoinHandle},
};

pub struct Processor<J, R> {
    rx: Receiver<R>,
    tx: Sender<J>,
    thread: Option<JoinHandle<Result<()>>>,
}

impl<J, R> Processor<J, R>
where
    J: Send + 'static,
    R: Send + 'static,
{
    pub fn new<F>(processor: F) -> Result<Self>
    where
        F: Send + 'static + FnOnce(Receiver<J>, Sender<R>) -> Result<()>,
    {
        let (txj, rxj) = channel();
        let (txr, rxr) = channel();
        Ok(Self {
            rx: rxr,
            tx: txj,
            thread: Some(
                thread::Builder::new()
                    .name("Bevy Main".into())
                    .spawn(move || processor(rxj, txr))?,
            ),
        })
    }

    pub fn process(&mut self, job: J) -> Result<R> {
        self.tx.send(job).map_err(|_| self.join_error())?;
        self.rx.recv().map_err(|_| self.join_error())
    }

    fn join_error(&mut self) -> BevyError {
        match self.thread.take() {
            Some(thread) => match thread.join() {
                Ok(Err(err)) => err,
                Ok(Ok(())) => "Failed to process render".into(),
                Err(payload) => panic_error(payload),
            },
            None => "Failed to process render".into(),
        }
    }
}

fn panic_error(payload: Box<dyn Any + Send>) -> BevyError {
    match payload.downcast::<String>() {
        Ok(message) => format!("Worker thread panicked: {message}").into(),
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(message) => format!("Worker thread panicked: {message}").into(),
            Err(_) => "Worker thread panicked".into(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Job(i32);
    #[derive(Debug, PartialEq)]
    struct Response(f32);

    #[test]
    fn test_process() {
        let mut processor = Processor::new(|rx: Receiver<Job>, tx: Sender<Response>| {
            for job in rx {
                tx.send(Response(job.0 as f32))?;
            }
            Ok(())
        })
        .unwrap();

        assert_eq!(processor.process(Job(7)).unwrap(), Response(7.0));
    }
}
