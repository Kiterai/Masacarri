import { defineStore } from "pinia";
import type { Comment } from "./models";
import type { Dayjs } from "dayjs";
import dayjs from "dayjs";
import { app_fetch } from "./utils";

export type ShowingComment = {
    comment_id: string,
    name: string,
    site_url?: string,
    date: Dayjs,
    content: string,
    children?: string[],
};

export const useCommentsStore = defineStore({
    id: "showing_comments",
    state: (): {
        comments: Map<string, Comment>,
        comment_showlist: string[],
    } => {
        return {
            comments: new Map(),
            comment_showlist: [],
        }
    },
    getters: {
        getComment(state) {
            return (id: string): ShowingComment | undefined => {
                const raw = state.comments.get(id);
                if (raw) {
                    return {
                        comment_id: raw.id,
                        name: raw.display_name,
                        site_url: raw.site_url,
                        date: dayjs(raw.created_time),
                        content: raw.content,
                        children: [],
                    };
                }
            }
        },
    },
    actions: {
        loadComment(page_id: string, index: number = 1, comment_per_page: number = 20) {
            app_fetch(`/api/pages/${page_id}/comments?index=${index}&num=${comment_per_page}`)
                .then((res: Comment[]) => {
                    this.comments.clear();
                    this.comment_showlist.length = 0;
                    for(const comment of res) {
                        this.comments.set(comment.id, comment);
                        this.comment_showlist.push(comment.id);
                    }
                });
        }
    },
});
