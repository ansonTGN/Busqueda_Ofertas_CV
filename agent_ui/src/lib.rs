// agent_ui/src/lib.rs
use anyhow::Result;
use async_nats::Client as NatsClient;
use crossbeam_channel::{unbounded, Receiver, Sender};
use eframe::egui;              // <- importa egui re-exportado por eframe
use futures::StreamExt;
use std::thread;
use tokio::runtime::Builder;

// ------------ Config por defecto (ajusta si prefieres pasarlos por parámetro) ------------
const DEFAULT_NATS_URL: &str = "nats://127.0.0.1:4222";
const DEFAULT_STATUS_SUBJECT: &str = "agent.ui.status"; // ¡Ojo! te suscribes luego a "agent.ui.status.<task_id>"


// ------------------------------- UI APP -----------------------------------

pub fn run() -> eframe::Result<()> {
    // Canal para pasar actualizaciones desde el listener NATS a la UI
    let (tx, rx) = unbounded::<String>();

    // Lanza el listener NATS en un hilo separado con su runtime Tokio
    let nats_url = DEFAULT_NATS_URL.to_string();
    let status_subject = DEFAULT_STATUS_SUBJECT.to_string();
    thread::spawn(move || {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        let _ = rt.block_on(listen_status(&nats_url, &status_subject, tx));
    });

    // Opciones de ventana en eframe 0.27 (initial_window_size -> viewport)
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Agent UI",
        native_options,
        Box::new(|_cc| Box::new(MyApp::new(rx))),
    )
}

struct MyApp {
    rx: Receiver<String>,
    last_status: String,
    busy: bool,
}

impl MyApp {
    fn new(rx: Receiver<String>) -> Self {
        Self {
            rx,
            last_status: "Esperando para iniciar...".to_string(),
            busy: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Drena mensajes pendientes del canal sin bloquear
        for msg in self.rx.try_iter() {
            self.last_status = msg;
            // Heurística simple para mostrar spinner si parece "en progreso"
            self.busy = self.last_status.contains("Iniciando")
                || self.last_status.contains("Procesando")
                || self.last_status.contains("Cargando")
                || self.last_status.contains("Running")
                || self.last_status.contains("Working");
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Estado del sistema");

            // Devuelve siempre egui::Response en cada rama:
            if self.busy {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(&self.last_status);
                })
                .response
            } else {
                ui.label(&self.last_status)
            };

            ui.separator();

            ui.label("Este panel recibe actualizaciones desde NATS:");
            ui.monospace(format!(
                "URL: {}\nSubject: {}",
                DEFAULT_NATS_URL, DEFAULT_STATUS_SUBJECT
            ));
        });
    }
}

// ---------------------------- NATS listener --------------------------------

async fn listen_status(nats_url: &str, subject: &str, tx: Sender<String>) -> Result<()> {
    let client = async_nats::connect(nats_url).await?;
    // Pasa subject como String (propietario) para evitar el problema de 'static:
    subscribe_and_forward(&client, subject.to_string(), tx).await
}

// Acepta subject como String (propietario) para cumplir con los requisitos de vida útil
async fn subscribe_and_forward(
    client: &NatsClient,
    subject: String,
    tx: Sender<String>,
) -> Result<()> {
    let mut sub = client.subscribe(subject).await?;
    while let Some(msg) = sub.next().await {
        if let Ok(text) = std::str::from_utf8(&msg.payload) {
            // Ignora errores de envío si la UI ya cerró
            let _ = tx.send(text.to_string());
        } else {
            let _ = tx.send("<payload binario>".to_string());
        }
    }
    Ok(())
}

