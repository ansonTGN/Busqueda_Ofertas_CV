use rust_xlsxwriter::{Workbook, Worksheet, XlsxError, Format, FormatAlign};
use std::path::PathBuf;

/// Estructuras de ejemplo: si ya tienes tus tipos, cambia solo las firmas.
#[derive(Debug, Clone)]
pub struct JobRow {
    pub title: String,
    pub company: String,
    pub location: String,
    pub source: String,
    pub source_url: String,
}

#[derive(Debug, Clone)]
pub struct ExportArgs {
    pub path: PathBuf,
    pub jobs: Vec<JobRow>,
}

/// Exporta a Excel sin bloquear Tokio (guardado síncrono dentro de `spawn_blocking`).
pub async fn export_jobs_to_xlsx(args: ExportArgs) -> Result<PathBuf, XlsxError> {
    let path = args.path.clone();

    let result: Result<PathBuf, XlsxError> = tokio::task::spawn_blocking(move || {
        // --- Libro y hoja ---
        let mut workbook = Workbook::new();
        let mut worksheet = workbook.add_worksheet();

        // `Format` usa "builder pattern": los setters consumen `self`.
        let header_fmt: Format = Format::new()
            .set_bold()
            .set_align(FormatAlign::Center);

        // Cabeceras (worksheet mutable)
        write_headers(&mut worksheet, &header_fmt)?;

        // Filas con datos
        for (row_idx, job) in args.jobs.iter().enumerate() {
            let r = (row_idx as u32) + 1;

            worksheet.write_string(r, 0, &job.title)?;
            worksheet.write_string(r, 1, &job.company)?;
            worksheet.write_string(r, 2, &job.location)?;
            worksheet.write_string(r, 3, &job.source)?;
            // `write_url` acepta `&str` (Into<Url>)
            worksheet.write_url(r, 4, job.source_url.as_str())?;
        }

        // Ajuste de columnas (opcional)
        worksheet.set_column_width(0, 28.0)?;
        worksheet.set_column_width(1, 24.0)?;
        worksheet.set_column_width(2, 20.0)?;
        worksheet.set_column_width(3, 16.0)?;
        worksheet.set_column_width(4, 46.0)?;

        // Guardado SÍNCRONO
        workbook.save(&path)?;

        Ok::<PathBuf, XlsxError>(path)
    })
    .await
    // `XlsxError::IoError` espera `std::io::Error`, no `String`.
    .map_err(|e| XlsxError::IoError(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("JoinError: {e}"),
    )))?;

    result
}

fn write_headers(worksheet: &mut Worksheet, header_fmt: &Format) -> Result<(), XlsxError> {
    worksheet.write_with_format(0, 0, "Título", header_fmt)?;
    worksheet.write_with_format(0, 1, "Empresa", header_fmt)?;
    worksheet.write_with_format(0, 2, "Ubicación", header_fmt)?;
    worksheet.write_with_format(0, 3, "Fuente", header_fmt)?;
    worksheet.write_with_format(0, 4, "URL", header_fmt)?;
    Ok(())
}

