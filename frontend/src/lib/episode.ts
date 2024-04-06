export interface Episode {
    id: number;
    channel_id: number;
    title: string;
    description: string;
    image: string;
    duration: number;
    listen: boolean;
    published_ad: Date;
    webpage_url: string;
    yt_id: string;
    created_at: Date;
    updated_at: Date;
}


