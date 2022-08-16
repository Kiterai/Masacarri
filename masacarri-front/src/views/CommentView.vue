<script setup lang="ts">
import CommentForm from "@/components/CommentForm.vue";
import CommentPost from "@/components/CommentPost.vue";
import { useCommentsStore } from '@/CommentsStore';
import { storeToRefs } from 'pinia';

const props = defineProps<{
  page_id: string
}>();

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

const { comment_replyto, comment_showlist, getComment } = storeToRefs(store);

</script>

<template>
  <CommentForm v-if="!comment_replyto"></CommentForm>

  <div class="post-list">
    <CommentPost v-for="comment_id in comment_showlist" :key="comment_id" :comment="getComment(comment_id)">
    </CommentPost>
  </div>

  <p>Powered by Masacarri</p>

</template>

<style scoped>
</style>
