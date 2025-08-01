use axum::http::{ StatusCode };
use axum::response::{ IntoResponse, Response };
use axum::{ body::Body, extract::{ Multipart } };
use std::{ fs::{ File }, io::{  Write } };

use tokio::fs::File as TokioFile;
use std::path::PathBuf;
use axum::extract::Path;
use tokio_util::io::ReaderStream;

    pub async fn upload_file(mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
        let mut file_name = String::new();
        let mut chunk_data = Vec::new();
        // let mut chunk_number = 0;
        // let mut total_chunks = 0;

        //["file"] request
        while let Some(field) = multipart.next_field().await.unwrap() {
            if field.name() == Some("file") {
                file_name = field.file_name().unwrap_or("uploaded.bin").to_string();
                chunk_data = field.bytes().await.unwrap_or_else(|_| Vec::new().into()).to_vec();
                
                let mut file = File::create(format!("./uploads/{}", &file_name)).unwrap();
                file.write_all(&chunk_data).unwrap();
            } 
        } 
        /* 
        while let Some(field) = match multipart.next_field().await {
           Ok(f) =>f,
           Err(err) => {
                return Err((StatusCode::BAD_REQUEST, format!("Error reading multipart field: {:?}", err)));
           }
        } {
            let field_name = field.name().unwrap_or_default().to_string();
            match field_name.as_str() {
                //["file_name"] request
                "file_name" => file_name = field.text().await.unwrap_or_default().to_string(),
                //["chunk_number"] request
                "chunk_number" => chunk_number = field.text().await.unwrap_or_default().parse().unwrap_or(0),
                //["total_chunks"] request
                "total_chunks" => total_chunks = field.text().await.unwrap_or_default().parse().unwrap_or(0),
                //chunk_data
                "chunk_data" => chunk_data = field.bytes().await.unwrap_or_else(|_| Vec::new().into()).to_vec(),
                _ => {}
            }
        }
         let mut file = File::create(format!("./uploads/{}", file_name)).unwrap();
        file.write_all(&chunk_data).unwrap();
        */
        if file_name.is_empty() || chunk_data.is_empty() {
             return Err(
                (StatusCode::BAD_REQUEST, format!("Failed to write chunk"))
            )
        }
         Ok(StatusCode::OK)
    }


pub async fn download_file(Path(filename): Path<String>) -> impl IntoResponse {
    let path = PathBuf::from(format!("./uploads/{}", filename));

    if !path.exists() {
        return StatusCode::NOT_FOUND.into_response();
    }

     match TokioFile::open(&path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);

            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/octet-stream")
                .body(body)
                .unwrap()
        }

        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}