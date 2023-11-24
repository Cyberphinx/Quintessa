use axum::Json;
use domain::education;
use sea_orm::{DatabaseConnection, Set};

use crate::core::{app_error::AppError, mapping_profiles::convert_education_to_response};

use super::{save::save_active_education, RequestCreateEducation, ResponseDataEducation};

pub async fn create(
    db: &DatabaseConnection,
    request_edu: RequestCreateEducation,
) -> Result<Json<ResponseDataEducation>, AppError> {
    let new_education = education::ActiveModel {
        resume_id: Set(request_edu.resume_id),
        school: Set(request_edu.school),
        degree: Set(request_edu.degree),
        ..Default::default()
    };

    let education = save_active_education(db, new_education).await?;

    let response = convert_education_to_response(education);

    Ok(Json(ResponseDataEducation { data: response }))
}
