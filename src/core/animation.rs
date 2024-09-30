use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;
use tokio::time::Duration;

pub struct Process {
    func: Pin<Box<dyn Future<Output = ()> + Send>>,
    text_during_execution: String,
    text_after_execution: String,
}

impl Process {
    pub fn new(func: impl Future<Output = ()> + Send + 'static) -> Self {
        Process {
            func: Box::pin(func),
            text_during_execution: String::new(),
            text_after_execution: String::new(),
        }
    }

    pub fn set_text_during_execution<S: Display>(mut self, text: S) -> Self {
        self.text_during_execution = text.to_string();
        self
    }

    /// Will display only in `parallel` execution.
    pub fn set_text_after_execution<S: Display>(mut self, text: S) -> Self {
        self.text_after_execution = text.to_string();
        self
    }
}

pub struct Animation {
    multi: MultiProgress,
    processes: Vec<Process>,
}

impl Animation {
    pub fn builder() -> Self {
        Self {
            multi: MultiProgress::new(),
            processes: vec![],
        }
    }

    pub fn add(mut self, process: Process) -> Self {
        self.processes.push(process);
        self
    }

    pub async fn invoke_sequentially(self, end_message: String) {
        let bar = self.multi.add(ProgressBar::new_spinner());
        bar.enable_steady_tick(Duration::from_millis(100));
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.blue} [{elapsed_precise}] {msg:.blue.underlined}",
            )
            .unwrap(),
        );

        for p in self.processes {
            bar.set_message(p.text_during_execution);
            p.func.await;
        }

        bar.set_style(
            ProgressStyle::with_template(
                "{prefix} {msg:.blue} in {elapsed:.green.bold.underlined}",
            )
            .unwrap(),
        );
        bar.set_prefix("✅");
        bar.finish_with_message(end_message);
    }

    pub async fn invoke_parallel(self) {
        let mut h = vec![];
        for p in self.processes {
            let bar = self.multi.add(ProgressBar::new_spinner());
            bar.enable_steady_tick(Duration::from_millis(100));
            bar.set_style(
                ProgressStyle::with_template(
                    "{spinner:.magenta} [{elapsed_precise}] {wide_msg:.magenta}",
                )
                .unwrap(),
            );

            h.push(tokio::spawn(async move {
                bar.set_message(p.text_during_execution);
                p.func.await;

                bar.set_style(
                    ProgressStyle::with_template(
                        "{prefix:.bold.dim} {msg:.green} in {elapsed:.cyan.underlined}",
                    )
                    .unwrap(),
                );
                bar.set_prefix("✅");
                bar.finish_with_message(p.text_after_execution);
                ()
            }))
        }

        for handle in h {
            handle.await.unwrap();
        }
    }
}
