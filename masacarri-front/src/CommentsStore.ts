import { defineStore } from "pinia";
import type { Comment, NewCommentRequest } from "./models";
import type { Dayjs } from "dayjs";
import dayjs from "dayjs";
import { app_fetch } from "./utils";

export type ShowingComment = {
    comment_id: string,
    name: string,
    site_url?: string,
    date: Dayjs,
    content: string,
    parent: string | undefined,
    count_replies: number,
    children?: ShowingComment[],
};

type PeginationContext = {
    item_count: number,
    item_per_page: number,
    index: number,
}

function toShowComment(raw: Comment): ShowingComment {
    return {
        comment_id: raw.id,
        name: raw.display_name,
        site_url: raw.site_url,
        date: dayjs(raw.created_time),
        content: raw.content,
        parent: raw.reply_to,
        count_replies: raw.count_replies,
        children: [],
    };
}

function latestPageIndex(comments_count: number, comments_per_page: number) {
    return ((comments_count + comments_per_page - 1) / comments_per_page) | 0;
}

export const useCommentsStore = defineStore({
    id: "showing_comments",
    state: (): {
        comments: Map<string, Comment>,
        main_pagination: PeginationContext,
        sub_pagination: PeginationContext | null,
        comments_count: number,
        comment_page_index: number,
        comment_showlist: ShowingComment[],
        comment_replyto: string | undefined,
        comment_shows_reply: string | undefined,
        comment_shows_context: string | undefined,
        page_id: string | undefined,
        page_loading: boolean,
    } => {
        return {
            comments: new Map(),
            main_pagination: {
                item_count: 0,
                index: 1,
                item_per_page: 7,
            },
            sub_pagination: null,
            comments_count: 0,
            comment_page_index: 1,
            comment_showlist: [],
            comment_replyto: undefined,
            comment_shows_reply: undefined,
            comment_shows_context: undefined,
            page_id: undefined,
            page_loading: true,
        }
    },
    getters: {
    },
    actions: {
        commentCountReload(page_id?: string) {
            if (!page_id)
                page_id = this.page_id;
            return app_fetch(`/api/pages/${page_id}/comments_count`)
                .then((res) => {
                    this.comments_count = res.count;
                });
        },
        loadPage(page_id: string, index: number | null = null, comment_per_page: number = 7) {
            const page_load =
                this.page_id != page_id
                    ? new Promise((resolve) => {
                        this.page_loading = true;
                        resolve(null);
                    })
                        .then(() => {
                            return this.commentCountReload(page_id);
                        })
                        .then((res) => {
                            this.page_id = page_id;
                        })
                    : Promise.resolve();

            return page_load
                .then(() => {
                    return this.loadComment(index, comment_per_page);
                })
                .then(() => {
                    this.page_loading = false;
                });
        },
        loadComment(index: number | null = null, comment_per_page: number = 7) {
            const realIndex = index ? index : latestPageIndex(this.comments_count, comment_per_page);
            return app_fetch(`/api/pages/${this.page_id}/comments?index=${realIndex}&num=${comment_per_page}`)
                .then((res: Comment[]) => {
                    this.comments.clear();
                    this.comment_showlist.length = 0;

                    for (const comment of res) {
                        this.comments.set(comment.id, comment);
                    }
                    for (const comment of res) {
                        this.comment_showlist.push(toShowComment(comment));
                    }
                    this.comment_page_index = realIndex;
                    if (this.sub_pagination) {
                        this.sub_pagination = null;
                    }

                    return res;
                });
        },
        loadCommentReply(replyto: string, index: number = 1, comment_per_page: number = 7) {
            return new Promise<Comment>((resolve) => {
                const target_comment = this.comments.get(replyto);
                if (target_comment) {
                    resolve(target_comment);
                } else {
                    app_fetch(`/api/pages/${this.page_id}/comments/${replyto}`)
                        .then((res: Comment) => {
                            resolve(res);
                        })
                }
            })
                .then((target_comment) => {
                    return app_fetch(`/api/pages/${this.page_id}/comments_count?replyto=${replyto}`)
                        .then((res) => {
                            const tmp = this.comments.get(target_comment.id);
                            if (tmp) {
                                tmp.count_replies = res.count;
                                this.comments.set(target_comment.id, tmp);
                            }
                            this.sub_pagination = {
                                index: index,
                                item_count: res.count,
                                item_per_page: 7,
                            }
                            return target_comment;
                        });
                })
                .then((target_comment) => {
                    return app_fetch(`/api/pages/${this.page_id}/comments?replyto=${replyto}&index=${index}&num=${comment_per_page}`)
                        .then((res: Comment[]) => {
                            this.comments.clear();
                            this.comment_showlist.length = 0;

                            this.comments.set(target_comment.id, target_comment);
                            for (const comment of res) {
                                this.comments.set(comment.id, comment);
                            }

                            const root = toShowComment(target_comment);
                            root.children = [];
                            for (const comment of res) {
                                root.children.push(toShowComment(comment));
                            }
                            this.comment_showlist.push(root);

                            this.comment_shows_context = undefined;
                            this.comment_shows_reply = target_comment.id;

                            return res;
                        });
                });
        },
        loadCommentContext(contextof: string, index: number = 1, comment_per_page: number = 7) {
            return new Promise<Comment>((resolve) => {
                const target_comment = this.comments.get(contextof);
                if (target_comment) {
                    resolve(target_comment);
                } else {
                    app_fetch(`/api/pages/${this.page_id}/comments/${contextof}`)
                        .then((res: Comment) => {
                            resolve(res);
                        })
                }
            })
                .then((target_comment) => {
                    return app_fetch(`/api/pages/${this.page_id}/comments_count?contextof=${contextof}`)
                        .then((res) => {
                            this.sub_pagination = {
                                index: index,
                                item_count: res.count,
                                item_per_page: 7,
                            }
                            return target_comment;
                        });
                })
                .then((target_comment) => {
                    return app_fetch(`/api/pages/${this.page_id}/comments?contextof=${contextof}&index=${index}&num=${comment_per_page}`)
                        .then((res: Comment[]) => {
                            this.comments.clear();
                            this.comment_showlist.length = 0;

                            this.comments.set(target_comment.id, target_comment);
                            for (const comment of res) {
                                this.comments.set(comment.id, comment);
                            }

                            const root = toShowComment(res[0]);
                            root.children = [];

                            for (const comment of res.slice(1)) {
                                root.children.push(toShowComment(comment));
                            }
                            this.comment_showlist.push(root);

                            this.comment_shows_reply = undefined;
                            this.comment_shows_context = target_comment.id;

                            return res;
                        });
                });
        },
        submitComment(comment: NewCommentRequest) {
            return app_fetch(`/api/pages/${this.page_id}/comments`, "POST", comment)
                .then((res: Comment) => {
                    return this.commentCountReload()
                        .then(() => {
                            this.comments.set(res.id, res);
                            if (res.reply_to) {
                                this.comment_replyto = undefined;
                            }
                            if (comment.reply_to) {
                                return this.loadCommentReply(comment.reply_to);
                            } else {
                                return this.loadComment();
                            }
                        })
                        .then(() => {
                            return res;
                        });
                })
        }
    },
});
