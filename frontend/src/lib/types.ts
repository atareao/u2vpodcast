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

export interface Response {
    status: boolean;
    status_code: number;
    message: string;
    user: User | null;
    data: Channel | Array<Channel> | Episode | Array<Episode> | null;
}

export interface Channel {
    id: number;
    url: string,
    title: string;
    active: boolean;
    description: string;
    image: string;
    first: Date;
    max: number;
    created_at: Date;
    updated_at: Date;
}

export interface Episode {
    id: number;
    channel_id: number;
    title: string;
    description: string;
    yt_id: string;
    webpage_url: string;
    published_at: Date;
    duration: string;
    image: string;
    listen: boolean;
    created_at: Date;
    updated_at: Date;
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
