use axum::{http::StatusCode, Json};
use domain::{
    media,
    project::{self, Entity as Projects},
};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryTrait,
};

use crate::core::{
    app_error::AppError,
    mapping_profiles::{convert_project_to_dto, convert_project_to_response},
};

use super::{
    project_params::ProjectParams, ResponseDataProjectDtos, ResponseDataProjects, ResponseProject,
    ResponseProjectDto, ResponseTotals,
};

pub async fn get_projects_totals(
    db: &DatabaseConnection,
) -> Result<Json<ResponseTotals>, AppError> {
    let response = Projects::find().count(db).await.map_err(|error| {
        eprintln!("Error getting all projects count: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error getting all projects count",
        )
    })?;

    Ok(Json(ResponseTotals { totals: response }))
}

pub async fn get_all_projects(
    db: &DatabaseConnection,
) -> Result<Json<ResponseDataProjects>, AppError> {
    let response = Projects::find()
        .all(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting all projects: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting all projects",
            )
        })?
        .into_iter()
        .map(convert_project_to_response)
        .collect::<Vec<ResponseProject>>();

    Ok(Json(ResponseDataProjects { data: response }))
}

pub async fn get_all_projects_with_media(
    project_params: ProjectParams,
    db: &DatabaseConnection,
) -> Result<Json<ResponseDataProjectDtos>, AppError> {
    let mut project_query = Projects::find();

    project_query = project_query.apply_if(Some(project_params.category), |query, value| {
        if value.to_owned().is_some_and(|x| !x.is_empty()) {
            query.filter(project::Column::Category.eq(value))
        } else {
            query
        }
    });

    let project = project_query
        .find_with_related(media::Entity)
        .all(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting all companies: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting all companies",
            )
        })?
        .into_iter()
        .map(convert_project_to_dto)
        .collect::<Vec<ResponseProjectDto>>();

    Ok(Json(ResponseDataProjectDtos { data: project }))
}
