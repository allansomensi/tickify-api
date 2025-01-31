#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("Failed to generate PDF: {0}")]
    PDFError(#[from] lopdf::Error),

    #[error("Failed to generate CSV: {0}")]
    CSVError(#[from] csv::Error),
}
