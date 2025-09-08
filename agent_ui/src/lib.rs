// agent_ui/src/lib.rs
use anyhow::Result;
use async_nats::Client as NatsClient;
use crossbeam_channel::{unbounded, Receiver, Sender};
use eframe::egui;
use futures::StreamExt;
use std::thread;
use tokio::runtime::Builder;

// Por defecto; pueden sobrescribirse con ENV
const DEFAULT_NATS_URL: &str = "nats://127.0.0.1:4222";
const DEFAULT_STATUS_SUBJECT: &str = "agents.status";

pub fn run() -> eframe::Result<()> {
    // Canal UI <— NATS
    let (tx, rx) = unbounded::<String>();

    // Usa ENV si existen
    let nats_url = std::env::var("NATS_URL").unwrap_or_else(|_| DEFAULT_NATS_URL.to_string());
    let status_subject =
        std::env::var("NATS_STATUS_SUBJECT").unwrap_or_else(|_| DEFAULT_STATUS_SUBJECT.to_string());

    // Clones para cada cierre (thread y UI)
    let nats_url_for_thread = nats_url.clone();
    let status_subject_for_thread = status_subject.clone();

    // Listener NATS en hilo separado
    thread::spawn(move || {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        let _ = rt.block_on(listen_status(&nats_url_for_thread, &status_subject_for_thread, tx));
    });

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };

    // Clones para pasar a la UI (evita mover los originales)
    let nats_url_for_ui = nats_url.clone();
    let status_subject_for_ui = status_subject.clone();

    eframe::run_native(
        "Agent UI",
        native_options,
        Box::new(|_cc| Box::new(MyApp::new(rx, nats_url_for_ui, status_subject_for_ui))),
    )
}

struct MyApp {
    rx: Receiver<String>,
    last_status: String,
    busy: bool,
    nats_url: String,
    status_subject: String,
}

impl MyApp {
    fn new(rx: Receiver<String>, nats_url: String, status_subject: String) -> Self {
        Self {
            rx,
            last_status: "Esperando para iniciar...".to_string(),
            busy: false,
            nats_url,
            status_subject,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for msg in self.rx.try_iter() {
            self.last_status = msg;
            self.busy = self.last_status.contains("Iniciando")
                || self.last_status.contains("Procesando")
                || self.last_status.contains("Cargando")
                || self.last_status.contains("Running")
                || self.last_status.contains("Working");
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Estado del sistema");

            if self.busy {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(&self.last_status);
                });
            } else {
                ui.label(&self.last_status);
            }

            ui.separator();
            ui.label("Origen de actualizaciones NATS:");
            ui.monospace(format!(
                "URL: {}\nSubject: {}",
                self.nats_url, self.status_subject
            ));
        });
    }
}

async fn listen_status(nats_url: &str, subject: &str, tx: Sender<String>) -> Result<()> {
    let client = async_nats::connect(nats_url).await?;
    subscribe_and_forward(&client, subject.to_string(), tx).await
}

async fn subscribe_and_forward(
    client: &NatsClient,
    subject: String,
    tx: Sender<String>,
) -> Result<()> {
    let mut sub = client.subscribe(subject).await?;
    while let Some(msg) = sub.next().await {
        if let Ok(text) = std::str::from_utf8(&msg.payload) {
            let _ = tx.send(text.to_string());
        } else {
            let _ = tx.send("<payload binario>".to_string());
        }
    }
    Ok(())
}

