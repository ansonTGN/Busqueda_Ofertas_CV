use super::Tool;
use anyhow::Result;
use async_trait::async_trait;
use rust_xlsxwriter::{Format, FormatAlign, Workbook, Worksheet, XlsxError};
use serde::Deserialize;
use serde_json::{json, Value};

pub struct ExcelWriterTool;

#[derive(Deserialize, Debug)]
struct ExcelWriterArgs {
    path: String,
    jobs: Vec<JobOffer>,
}

#[derive(Deserialize, Debug)]
struct JobOffer {
    title: String,
    company: String,
    location: String,
    contact: String,
    source_url: String,
}

#[async_trait]
impl Tool for ExcelWriterTool {
    fn name(&self) -> &'static str {
        "excel_writer"
    }

    fn description(&self) -> &'static str {
        "Escribe una lista de ofertas de trabajo en un archivo .xlsx."
    }

    async fn execute(&self, args_json: &str) -> Result<Value> {
        let args: ExcelWriterArgs = serde_json::from_str(args_json)?;

        let mut workbook = Workbook::new();
        let mut worksheet = workbook.add_worksheet();

        self.write_headers(&mut worksheet)?;

        for (row_num, job) in args.jobs.iter().enumerate() {
            let r = row_num as u32 + 1;
            worksheet.write_string(r, 0, &job.title)?;
            worksheet.write_string(r, 1, &job.company)?;
            worksheet.write_string(r, 2, &job.location)?;
            worksheet.write_string(r, 3, &job.contact)?;
            // Importante: pasar &str, no &String
            worksheet.write_url(r, 4, job.source_url.as_str())?;
        }

        // Ancho de columnas razonable
        worksheet.set_column_width(0, 28.0)?;
        worksheet.set_column_width(1, 24.0)?;
        worksheet.set_column_width(2, 20.0)?;
        worksheet.set_column_width(3, 18.0)?;
        worksheet.set_column_width(4, 46.0)?;

        // Guardado síncrono (si prefieres no bloquear, lo envolvemos con spawn_blocking)
        workbook.save(&args.path)?;

        Ok(json!({ "status": "success", "path": args.path }))
    }
}

impl ExcelWriterTool {
    fn write_headers(&self, worksheet: &mut Worksheet) -> Result<(), XlsxError> {
        let header_fmt = Format::new().set_bold().set_align(FormatAlign::Center);

        worksheet.write_with_format(0, 0, "Puesto", &header_fmt)?;
        worksheet.write_with_format(0, 1, "Empresa", &header_fmt)?;
        worksheet.write_with_format(0, 2, "Ubicación", &header_fmt)?;
        worksheet.write_with_format(0, 3, "Contacto", &header_fmt)?;
        worksheet.write_with_format(0, 4, "Fuente", &header_fmt)?;
        Ok(())
    }
}


