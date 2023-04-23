use rocket::{
    http::{ContentType, Status},
    response::Responder,
    Request, Response,
};
use serde_json::json;

pub struct ApiErrorResponse {
    pub detail: String,
    pub status: Status,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiErrorResponse {
    fn respond_to(self, request: &'r Request<'_>) -> Result<Response<'o>, Status> {
        if let Ok(response) = json!({"status_code": self.status.to_string(),"message": self.detail})
            .respond_to(request)
        {
            Response::build_from(response)
                .status(self.status)
                .header(ContentType::JSON)
                .ok()
        } else {
            Response::build()
                .status(Status::InternalServerError)
                .header(ContentType::JSON)
                .ok()
        }
    }
}
