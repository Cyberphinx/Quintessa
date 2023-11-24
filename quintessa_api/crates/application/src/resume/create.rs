use axum::Json;
use domain::resume;
use sea_orm::{DatabaseConnection, Set};

use crate::{
    core::{app_error::AppError, mapping_profiles::convert_resume_to_response},
    resume::save::save_active_resume,
};

use super::{RequestCreateResume, ResponseDataResume};

pub async fn create(
    db: &DatabaseConnection,
    request_resume: RequestCreateResume,
) -> Result<Json<ResponseDataResume>, AppError> {
    let new_resume = resume::ActiveModel {
        name: Set(request_resume.name),
        email: Set(request_resume.email),
        mobile: Set(request_resume.mobile),
        website: Set(request_resume.website),
        birthdate: Set(request_resume.birthdate),
        nationality: Set(request_resume.nationality),
        top_skills: Set(request_resume.top_skills),
        languages: Set(request_resume.languages),
        certifications: Set(request_resume.certifications),
        snippets: Set(request_resume.snippets),
        job_title: Set(request_resume.job_title),
        address: Set(request_resume.address),
        summary: Set(request_resume.summary),
        workshops: Set(request_resume.workshops),
        ..Default::default()
    };

    let resume = save_active_resume(db, new_resume).await?;

    let response = convert_resume_to_response(resume);

    Ok(Json(ResponseDataResume { data: response }))
}
