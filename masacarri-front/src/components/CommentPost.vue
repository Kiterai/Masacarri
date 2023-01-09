<script setup lang="ts">
import { computed } from '@vue/reactivity';
import { marked } from 'marked';
import { useCommentsStore, type ShowingComment } from '@/CommentsStore';
import { storeToRefs } from 'pinia';
import CommentForm from './CommentForm.vue';
import sanitizeHtml from 'sanitize-html';

const store = useCommentsStore();
const { comment_replyto } = storeToRefs(store);

const props = defineProps<{
    comment: ShowingComment,
    hide_buttons?: boolean,
    is_admin?: boolean,
}>();

const emit = defineEmits<{
    (e: 'beginReplyClicked', id: string): void,
    (e: 'cancelReplyClicked', id: string): void,
    (e: 'showRepliesClicked', id: string): void,
    (e: 'showContextsClicked', id: string): void,
    (e: 'markCommentClicked', id: string, is_spam: boolean): void,
}>();

const content = computed(() => {
    const parsed = marked.parse(props.comment.content);
    const sanitized = sanitizeHtml(parsed);
    return sanitized;
});

const date_str = computed(() => {
    return props.comment.date.format('YYYY-MM-DD HH:mm:ss');
});

function beginReplyClicked() {
    emit("beginReplyClicked", props.comment.comment_id);
}
function childBeginReplyClicked(id: string) {
    emit("beginReplyClicked", id);
}

function cancelReplyClicked() {
    emit("cancelReplyClicked", props.comment.comment_id);
}
function childCancelReplyClicked(id: string) {
    emit("cancelReplyClicked", id);
}

function showRepliesClicked() {
    emit("showRepliesClicked", props.comment.comment_id);
}
function childShowRepliesClicked(id: string) {
    emit("showRepliesClicked", id);
}

function showContextsClicked() {
    emit("showContextsClicked", props.comment.comment_id);
}
function childShowContextsClicked(id: string) {
    emit("showContextsClicked", id);
}

function toReplyto() {
    if (props.comment.parent)
        store.loadCommentReply(props.comment.parent);
}

function markCommentAsSpamClicked() {
    emit("markCommentClicked", props.comment.comment_id, true);
}

function unmarkCommentAsSpamClicked() {
    emit("markCommentClicked", props.comment.comment_id, false);
}

</script>

<template>
    <div class="post"
        :data-current-focus="store.comment_shows_reply == props.comment.comment_id || store.comment_shows_context == props.comment.comment_id"
        :data-spam="props.comment.is_spam">
        <div class="post-meta">
            <a v-if="props.comment.parent" class="post-isreply" @click="toReplyto">返信:</a>
            <a class="post-name" :href="props.comment.site_url" target="_blank" rel="noopener noreferrer">{{ props.comment.name }}</a>
            <time class="post-date" :datetime="props.comment.date.toISOString()">{{ date_str }}</time>
            <div v-if="props.is_admin">
                <button v-if="props.comment.is_spam" @click="unmarkCommentAsSpamClicked">[unmark as spam]</button>
                <button v-else @click="markCommentAsSpamClicked">[mark as spam]</button>
            </div>
        </div>
        <div class="post-content" v-html="content"></div>
        <div class="btns" v-if="!props.hide_buttons">
            <button class="btn btn-reply" v-if="comment_replyto == comment.comment_id"
                @click="cancelReplyClicked">返信をキャンセル</button>
            <button class="btn btn-reply" v-else @click="beginReplyClicked">返信する</button>
            <span v-if="comment.count_replies > 0" class="btn-separator"> | </span>
            <button class="btn" @click="showRepliesClicked" v-if="comment.count_replies > 0">{{ comment.count_replies
            }}件の返信</button>
            <span v-if="props.comment.parent" class="btn-separator"> | </span>
            <button v-if="props.comment.parent" class="btn" @click="showContextsClicked">文脈を読む</button>
        </div>
        <CommentForm v-if="comment_replyto == comment.comment_id" :comment_replyto="comment.comment_id"></CommentForm>
    </div>
    <div v-if="props.comment.children" class="post-list">
        <CommentPost v-for="child in props.comment.children" :key="child.comment_id" :comment="child"
            @begin-reply-clicked="childBeginReplyClicked" @cancel-reply-clicked="childCancelReplyClicked"
            @show-replies-clicked="childShowRepliesClicked" @show-contexts-clicked="childShowContextsClicked">
        </CommentPost>
    </div>

</template>

<style scoped>
.post {
    border-left: 0.2rem rgb(229, 229, 235) solid;
    padding: 0.5rem;
    transition: border-color 0.3s, background-color 0.3s;
    margin-bottom: 0.3rem;
}

.post:hover {
    border-left: 0.2rem rgb(180, 180, 201) solid;
    background-color: rgba(0, 0, 0, 0.05);
}

.post[data-current-focus=true] {
    border-left: 0.2rem #ffe9b7 solid;
    background-color: #ffdd8f54;
}

.post-name {
    font-weight: bold;
}

.post-meta {
    color: #888;
}

.post-isreply {
    margin-right: 0.5em;
    color: #888;
    cursor: pointer;
}

.post-isreply:hover {
    text-decoration: underline;
}

.post-date {
    margin-left: 0.5em;
}

.post-content {
    padding: 0.5em;
}

.btns {
    margin-left: 0.5em;
}

.btn {
    /* margin-left: 0.5em; */
    text-decoration: none;
    color: rgb(13, 139, 97);
}

.btn:hover {
    text-decoration: underline;
}

.btn-separator {
    color: rgba(0, 0, 0, 0.25);
}

.post-list {
    margin-left: 1em;
}
</style>

<style>
[data-spam="true"] .post-content {
    color: #AAA;
}

.post-content ul,
.post-content ol {
    margin-left: 1em;
}

.post-content hr {
    margin: 0.5em 0;
}

.post-content p:not(:last-child) {
    margin-bottom: 0.8em;
}

.post-content table {
    margin: 0.5em 0;
}

.post-content thead {
    border-bottom: 1px solid #000;
}

.post-content td,
.post-content th {
    border-left: 1px solid #000;
    padding: 0.3em;
}

.post-content td,
.post-content th {
    border-left: 1px solid #000;
    padding: 0.3em;
}

.post-content td:first-child,
.post-content th:first-child {
    border-left: none;
}
</style>
