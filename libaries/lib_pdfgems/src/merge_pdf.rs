use lopdf::Document;
use std::{fs::File, path::PathBuf};

use crate::ultilities::{merge_pdf, read_base64, PdfError};

pub fn merge_pdf_handler(vec_base64: Vec<String>, file_path: PathBuf) -> Result<i32, PdfError> {
    let vec_documents = vec_base64
        .iter()
        .map(|x| read_base64(x))
        .collect::<Result<Vec<Document>, _>>()
        .map_err(|_| PdfError {
            message: "Pages root not found.".into(),
        })?;
    let mut documents = merge_pdf(vec_documents).map_err(|_| PdfError {
        message: "Pages root not found.".into(),
    })?; // Propagates the error if merge_pdf fails
    let mut file = File::create(file_path).map_err(|_| PdfError {
        message: "Pages root not found.".into(),
    })?;
    documents.save_to(&mut file).map_err(|_| PdfError {
        message: "Pages root not found.".into(),
    })?;

    Ok(200)
}
