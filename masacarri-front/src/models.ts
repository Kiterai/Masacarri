export type Comment = {
    id: string,
    page_id: string,
    reply_to?: string,
    display_name: string,
    site_url?: string,
    content: string,
    is_spam?: boolean,
    count_replies: number,
    created_time: string,
};

export type Page = {
    id: string,
    title: string,
    page_url: string,
    published: boolean,
};

export type NewCommentRequest = {
    reply_to?: string,
    display_name: string,
    site_url?: string,
    mail_addr?: string,
    content: string,
    delete_key?: string,
}
