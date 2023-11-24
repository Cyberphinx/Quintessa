use axum::Json;
use reqwest::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};

use crate::core::{
    app_error::AppError,
    mapping_profiles::{convert_education_to_response, convert_work_to_response},
};

use domain::{education::Entity as Educations, resume::Entity as Resumes, work::Entity as Works};

use super::{ResponseDataResumeDto, ResponseResumeDto};

pub async fn details(
    resume_id: i32,
    db: &DatabaseConnection,
) -> Result<Json<ResponseDataResumeDto>, AppError> {
    let resume = Resumes::find_by_id(resume_id)
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting resume by id {} : {:?}", resume_id, error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting resume, please try again later",
            )
        })?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "The resume is not found"))?;

    let works = resume
        .find_related(Works)
        .all(db)
        .await
        .map_err(|error| {
            eprintln!("Problem finding related works: {}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Problem finding related works",
            )
        })?
        .into_iter()
        .map(convert_work_to_response)
        .collect::<Vec<_>>();

    let educations = resume
        .find_related(Educations)
        .all(db)
        .await
        .map_err(|error| {
            eprintln!("Problem finding related educations: {}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Problem finding related educations",
            )
        })?
        .into_iter()
        .map(convert_education_to_response)
        .collect::<Vec<_>>();

    let response = ResponseResumeDto {
        id: resume.id,
        name: resume.name,
        email: resume.email,
        mobile: resume.mobile,
        website: resume.website,
        birthdate: resume.birthdate,
        nationality: resume.nationality,
        top_skills: resume.top_skills,
        languages: resume.languages,
        certifications: resume.certifications,
        snippets: resume.snippets,
        job_title: resume.job_title,
        address: resume.address,
        summary: resume.summary,
        workshops: resume.workshops,
        works,
        educations,
    };

    Ok(Json(ResponseDataResumeDto { data: response }))
}
