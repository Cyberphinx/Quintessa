export interface SupabaseParams {
    url: string;
    key: string;
}

export interface LoginDataResponse {
    data: LoginResponse;
}
export interface LoginResponse {
    id: number;
    email: string;
    role: string;
    token: string;
}

export interface Project {
    id: number;
    created_at: Date;
    updated_at: Date;
    deleted_at: Date;
    name: string;
    category: string;
    sub_category: string;
    description: string;
    tasks: string;
    sector: string;
    location: string;
    address: string;
    client: string;
    start_date: string;
    completion_date: string;
    architect: string;
    main_contractor: string;
    project_manager: string;
    structural_engineer: string;
    services_engineer: string;
    media: Media[];
}

export interface Media {
    id: number;
    project_id: number;
    created_at: Date;
    updated_at: Date;
    deleted_at: Date;
    media_type: string;
    url: string;
    caption: string;
    description: string;
}

export interface SignedMedia {
    error: string;
    path: string;
    signedURL: string;
}
