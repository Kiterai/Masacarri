<script setup lang="ts">
import { computed } from '@vue/reactivity';
import { marked } from 'marked';
import { useCommentsStore, type ShowingComment } from '@/CommentsStore';
import { storeToRefs } from 'pinia';
import CommentForm from './CommentForm.vue';

const store = useCommentsStore();
const { comment_replyto } = storeToRefs(store);

const props = defineProps<{
    comment: ShowingComment,
    hide_buttons?: boolean,
}>();

const emit = defineEmits<{
    (e: 'beginReplyClicked', id: string): void,
    (e: 'cancelReplyClicked', id: string): void,
    (e: 'showRepliesClicked', id: string): void,
    (e: 'showContextsClicked', id: string): void,
}>();

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
    if(props.comment.parent)
        store.loadCommentReply(props.comment.parent);
}

</script>

<template>
    <div class="post">
        <div class="post-meta">
            <a v-if="props.comment.parent" class="post-isreply" @click="toReplyto">返信:</a>
            <a class="post-name">{{ props.comment.name }}</a>
            <time class="post-date" :datetime="props.comment.date.toISOString()">{{ date_str }}</time>
        </div>
        <div class="post-content" v-html="marked.parse(props.comment.content)"></div>
        <div class="btns" v-if="!props.hide_buttons">
            <button class="btn btn-reply" v-if="comment_replyto == comment.comment_id"
                @click="cancelReplyClicked">返信をキャンセル</button>
            <button class="btn btn-reply" v-else @click="beginReplyClicked">返信する</button>
            <button class="btn" @click="showRepliesClicked" v-if="comment.count_replies > 0">{{ comment.count_replies }}件の返信</button>
            <button v-if="props.comment.parent" class="btn" @click="showContextsClicked">文脈を読む</button>
        </div>
        <CommentForm v-if="comment_replyto == comment.comment_id" :comment_replyto="comment.comment_id"></CommentForm>
    </div>
    <div v-if="props.comment.children" class="post-list">
        <CommentPost v-for="child in props.comment.children" :key="child.comment_id" :comment="child"
            @begin-reply-clicked="childBeginReplyClicked" @cancel-reply-clicked="childCancelReplyClicked"
            @show-replies-clicked="childShowRepliesClicked" @show-contexts-clicked="childShowContextsClicked"></CommentPost>
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

.btn {
    margin-left: 0.5em;
    text-decoration: underline;
    color: rgb(13, 139, 97);
}

.post-list {
    margin-left: 1em;
}
</style>
