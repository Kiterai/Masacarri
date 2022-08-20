export type Comment = {
    id: string,
    page_id: string,
    reply_to?: string,
    display_name: string,
    site_url?: string,
    content: string,
    count_replies: number,
    created_time: string,
};

export type Page = {
    id: string,
    title: string,
    page_url: string,
    published: boolean,
};
