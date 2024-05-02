export interface Topic {
    id: number | string;
    title: string;
    url: string;
}

export interface ApiResponse {
    message?: string;
    status?: string;
}

export interface CustomError {
    error?: string;
    id?: number;
}

export interface RegenerateTokenRequestBody {
    username: string;
}

export interface LoginRequestBody extends RegenerateTokenRequestBody {
    password: string;
}

export interface RegisterRequestBody extends LoginRequestBody {
    username: string;
}
export interface PasswordChange {
    token: string;
    password: string;
}

interface UserProfile {
    id: string;
    user_id: string;
    phone_number: string | null;
    birth_date: string | null;
    github_link: string | null;
}

export interface User {
    id: number;
    name: string;
    role: string;
    active: boolean;
}

type Status = 'IDLE' | 'LOADING' | 'NAVIGATING';

export interface Loading {
    status: Status;
    message: string;
}

export interface SeriesAndArticles {
    id: string;
    name: string;
    image: string;
    articles: Array<Topic>;
}
