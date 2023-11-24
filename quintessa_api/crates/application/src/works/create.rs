use axum::Json;
use domain::work;
use sea_orm::{DatabaseConnection, Set};

use crate::core::{app_error::AppError, mapping_profiles::convert_work_to_response};

use super::{save::save_active_work, RequestCreateWork, ResponseDataWork};

pub async fn create(
    db: &DatabaseConnection,
    request_work: RequestCreateWork,
) -> Result<Json<ResponseDataWork>, AppError> {
    let new_work = work::ActiveModel {
        resume_id: Set(request_work.resume_id),
        company_name: Set(request_work.company_name),
        position: Set(request_work.position),
        duration: Set(request_work.duration),
        location: Set(request_work.location),
        projects: Set(request_work.projects),
        ..Default::default()
    };

    let work = save_active_work(db, new_work).await?;

    let response = convert_work_to_response(work);

    Ok(Json(ResponseDataWork { data: response }))
}
