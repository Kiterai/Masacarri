<script setup lang="ts">
import { computed } from '@vue/reactivity';
import type { Dayjs } from "dayjs";
import { marked } from 'marked';
import type { ShowingComment } from '@/CommentsStore';

const props = defineProps<{
    comment: ShowingComment,
    hide_buttons?: boolean,
}>();

const emit = defineEmits<{
    (e: 'beginReplyClicked', id: string): void,
    (e: 'showRepliesClicked', id: string): void,
    (e: 'showContextsClicked', id: string): void,
}>();

const date_str = computed(() => {
    return props.comment.date.format('YYYY-MM-DD HH:mm:ss');
});

function beginReplyClicked() {
    emit("beginReplyClicked", props.comment.comment_id);
}

function showRepliesClicked() {
    emit("showRepliesClicked", props.comment.comment_id);
}

function showContextsClicked() {
    emit("showContextsClicked", props.comment.comment_id);
}

</script>

<template>
    <div class="post">
        <div class="post-meta">
            <a class="post-name">{{ props.comment.name }}</a>
            <time class="post-date" :datetime="props.comment.date.toISOString()">{{ date_str }}</time>
        </div>
        <div class="post-content" v-html="marked.parse(props.comment.content)"></div>
        <div class="btns" v-if="!props.hide_buttons">
            <button class="btn btn-reply" @click="beginReplyClicked">返信する</button>
            <button class="btn" @click="showRepliesClicked">返信一覧</button>
            <button class="btn" @click="showContextsClicked">文脈を読む</button>
        </div>
    </div>
    <!-- <div v-if="props.comment.children" class="post-list">
        <CommentPost v-for="child in props.comment.children" :key="child" :comment="child"></CommentPost>
    </div> -->

</template>

<style scoped>
.post {
    border-left: 0.2rem rgb(229, 229, 235) solid;
    padding: 0.5rem;
    transition: border-color 0.3s;
    margin-bottom: 0.3rem;
}

.post:hover {
    border-left: 0.2rem rgb(203, 203, 255) solid;
}

.post-name {
    font-weight: bold;
}

.post-date {
    margin-left: 0.5em;
    color: #888;
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
