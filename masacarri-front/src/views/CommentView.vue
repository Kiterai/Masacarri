<script setup lang="ts">
import dayjs from 'dayjs';
import CommentForm from "@/components/CommentForm.vue";
import CommentPost from "@/components/CommentPost.vue";
import { ref } from 'vue';
import type { Comment } from '@/models';
import { app_fetch } from '@/utils';
import { useCommentsStore } from '@/CommentsStore';
import { storeToRefs } from 'pinia';

const props = defineProps<{
  page_id: string
}>();

// const comments = ref<Comment[]>([]);
// const comment_replyto = ref<Comment | null>(null);

// function load_comments() {
//   app_fetch(`/api/pages/${props.page_id}/comments`)
//     .then((res: Comment[]) => {
//       comments.value = res;
//     });
// }
// load_comments();

// function on_comment_submit(data: Comment) {
//   comments.value.unshift(data);
//   comment_replyto.value = null;
// }

// function on_begin_reply_clicked(id: string) {
//   const comment = comments.value.find(comment => comment.id == id);
//   if (comment) {
//     comment_replyto.value = comment;
//   }
// }

// function show_replies(id: string) {
//   app_fetch(`/api/pages/${props.page_id}/comments?replyto=${id}`)
//     .then((res: Comment[]) => {
//       console.log(res);
//       // comments.value = res;
//     });
// }

const store = useCommentsStore();

store.loadComment(props.page_id)

const { comment_showlist, getComment } = storeToRefs(store);

</script>

<template>
  <!-- <div v-if="comment_replyto">
    <div>このコメントに返信:</div>
    <CommentPost :hide_buttons="true"
      :comment="{ comment_id: comment_replyto.id, name: comment_replyto.display_name, date: dayjs(comment_replyto.created_time), content: comment_replyto.content }">
    </CommentPost>
  </div> -->
  <!-- <CommentForm :page_id="page_id" :reply_to_id="comment_replyto?.id" @comment-submit="on_comment_submit"></CommentForm> -->

  <div class="post-list">
    <CommentPost v-for="comment_id in comment_showlist" :key="comment_id" :comment="getComment(comment_id)">
    </CommentPost>
  </div>

  <p>Powered by Masacarri</p>

</template>

<style scoped>
</style>
