use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use actix_multipart::{Field, Multipart};
use actix_web::{App, FromRequest, HttpRequest, HttpServer, Responder};
use actix_web::dev::Payload;
use actix_web::web::{Bytes, post};
use std::str::FromStr;

struct MultipartRequestWithFile {
    file_name: String,
    file_content: Vec<Bytes>,
    other_param: String,
}

impl MultipartRequestWithFile {
    async fn from_multipart(mut multipart: Multipart) -> Result<Self, <Self as FromRequest>::Error> {
        // The request might not contain any of the parameters, so need to check their presence
        let mut other_param: Option<String> = None;
        let mut file_name: Option<String> = None;
        let mut file_content: Option<Vec<Bytes>> = None;

        // Iterate over all the multipart fields
        while let Some(mut field) = multipart.try_next().await? {
            let content_disposition = field.content_disposition().unwrap();
            let field_name = content_disposition.get_name().unwrap();

            // Parse the different parameters expected in the multipart request
            match field_name {
                "other_param" => {
                    other_param = Self::read_string(&mut field).await;
                }
                // Assuming the file parameter is called 'file'
                "file" => {
                    file_name = content_disposition.get_filename().map(String::from);

                    // Need to collect the whole file content here instead of passing the stream of bytes to the 'MultipartRequestWithFile' struct
                    // because 'Field' is not 'Send'
                    file_content = Some(field.map(|chunk| chunk.unwrap()).collect::<Vec<Bytes>>().await);
                }
                // Ignore other parameters
                _ => ()
            }
        }

        // Assert the required fields are present
        if other_param.is_some() && file_name.is_some() && file_content.is_some() {
            Ok(MultipartRequestWithFile {
                other_param: other_param.unwrap(),
                file_name: file_name.unwrap(),
                file_content: file_content.unwrap(),
            })
        } else {
            Err(actix_web::Error::from("Invalid request")) // TODO - create a custom validation error which implements the actix_web::ResponseError trait
        }
    }

    async fn read_string(field: &mut Field) -> Option<String> {
        let bytes = field.try_next().await;

        if let Ok(Some(bytes)) = bytes {
            String::from_utf8(bytes.to_vec()).ok()
        } else {
            None
        }
    }
}

// By implementing FromRequest, we can directly declare our struct as a parameter for an actix handler function
impl FromRequest for MultipartRequestWithFile {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // get a future for a Multipart struct from the request
        let multipart_future = Multipart::from_request(req, payload);

        // As this is not an async function, we cannot use 'await'.
        // Instead, we create future from this async block and return a pinned Box containing our future.
        // This is because currently, traits cannot declare async functions, so instead the FromRequest trait declares a non-async function which returns a Future instead.
        let future = async {
            // Inside of this async block we are able to use 'await'
            let multipart = multipart_future.await?;

            // Await our async function containing the actual logic
            Self::from_multipart(multipart).await
        };

        Box::pin(future)
    }
}

// Directly declare our custom struct as a handle parameter
async fn handle_multipart(req_with_file: MultipartRequestWithFile) -> impl Responder {
    todo!()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/file-upload", post().to(handle_multipart))
    })
        .bind("localhost:7878")?
        .run()
        .await
}
