use domain::{education, media, project, refresh_token, resume, user, work};
use itertools::Itertools;

use crate::{
    educations::ResponseEducation,
    media::ResponseMedia,
    projects::{ResponseProject, ResponseProjectDto},
    refresh_token::ResponseRefreshToken,
    resume::ResponseResume,
    users::ResponseUser,
    works::ResponseWork,
};

pub fn convert_education_to_response(education: education::Model) -> ResponseEducation {
    ResponseEducation {
        id: education.id,
        resume_id: education.resume_id,
        school: education.school,
        degree: education.degree,
    }
}

pub fn convert_project_to_response(project: project::Model) -> ResponseProject {
    ResponseProject {
        id: project.id,
        created_at: project.created_at,
        updated_at: project.updated_at,
        deleted_at: project.deleted_at,
        name: project.name,
        category: project.category,
        sub_category: project.sub_category,
        description: project.description,
        tasks: project.tasks,
        sector: project.sector,
        location: project.location,
        address: project.address,
        client: project.client,
        start_date: project.start_date,
        completion_date: project.completion_date,
        architect: project.architect,
        main_contractor: project.main_contractor,
        project_manager: project.project_manager,
        structural_engineer: project.structural_engineer,
        services_engineer: project.services_engineer,
    }
}

pub fn convert_project_to_dto(
    project_with_media: (project::Model, Vec<media::Model>),
) -> ResponseProjectDto {
    let media = project_with_media
        .1
        .into_iter()
        .map(convert_media_to_response)
        .collect_vec();

    ResponseProjectDto {
        id: project_with_media.0.id,
        created_at: project_with_media.0.created_at,
        updated_at: project_with_media.0.updated_at,
        deleted_at: project_with_media.0.deleted_at,
        name: project_with_media.0.name,
        category: project_with_media.0.category,
        sub_category: project_with_media.0.sub_category,
        description: project_with_media.0.description,
        tasks: project_with_media.0.tasks,
        sector: project_with_media.0.sector,
        location: project_with_media.0.location,
        address: project_with_media.0.address,
        client: project_with_media.0.client,
        start_date: project_with_media.0.start_date,
        completion_date: project_with_media.0.completion_date,
        architect: project_with_media.0.architect,
        main_contractor: project_with_media.0.main_contractor,
        project_manager: project_with_media.0.project_manager,
        structural_engineer: project_with_media.0.structural_engineer,
        services_engineer: project_with_media.0.services_engineer,
        media: Some(media),
    }
}

pub fn convert_resume_to_response(resume: resume::Model) -> ResponseResume {
    ResponseResume {
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
    }
}

pub fn convert_media_to_response(media: media::Model) -> ResponseMedia {
    ResponseMedia {
        id: media.id,
        project_id: media.project_id,
        created_at: media.created_at,
        updated_at: media.updated_at,
        deleted_at: media.deleted_at,
        media_type: media.media_type,
        url: media.url,
        caption: media.caption,
        description: media.description,
    }
}

pub fn convert_user_to_response(user: user::Model) -> ResponseUser {
    ResponseUser {
        id: user.id,
        email: user.email,
        role: user.role,
        token: user.token.unwrap(),
    }
}

pub fn convert_refresh_token_to_response(
    refresh_token: refresh_token::Model,
) -> ResponseRefreshToken {
    ResponseRefreshToken {
        id: refresh_token.id,
        user_id: refresh_token.user_id,
        token: refresh_token.token,
        expires: refresh_token.expires,
        revoked: refresh_token.revoked,
    }
}

pub fn convert_work_to_response(work: work::Model) -> ResponseWork {
    ResponseWork {
        id: work.id,
        resume_id: work.resume_id,
        company_name: work.company_name,
        position: work.position,
        duration: work.duration,
        location: work.location,
        projects: work.projects,
    }
}
