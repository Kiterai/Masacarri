<script setup lang="ts">
import CommentForm from "@/components/CommentForm.vue";
import CommentPost from "@/components/CommentPost.vue";
import { useCommentsStore } from '@/CommentsStore';
import { storeToRefs } from 'pinia';
import { computed } from "@vue/reactivity";
import type { Comment } from "@/models";
import { app_fetch_admin, repository_url } from "@/utils";
import { ref } from "vue";

const store = useCommentsStore();

const props = defineProps<{
  page_id: string,
  is_admin?: boolean,
}>();

const query_params = new URLSearchParams(location.search);
const custom_style = ref({
  color: query_params.has('color') ? '#' + query_params.get('color') : undefined
});

function on_comment_submit(data: Comment) {
  if (data.reply_to) {
    store.comment_replyto = undefined;
  }
}

function on_begin_reply_clicked(id: string) {
  store.comment_replyto = id;
}

function on_cancel_reply_clicked(id: string) {
  store.comment_replyto = undefined;
}

function show_replies(id: string) {
  store.loadCommentReply(id);
}
function show_contexts(id: string) {
  store.loadCommentContext(id);
}

function mark_comment(id: string, is_spam: boolean) {
  app_fetch_admin(`/api/pages/${props.page_id}/comments/${id}`, "PATCH", {
    spam: is_spam,
  }).then(() => {
    alert("succeeded to mark comment")
  })
  .catch(() => {
    alert("failed to mark comment")
  });
}

store.loadPage(props.page_id);

const commentsPerPage = 7;

const linkCommentPageIndices = computed(() => {
  const indices: Set<number> = new Set();
  const lastIndex = Math.max(1, ((store.comments_count + commentsPerPage - 1) / commentsPerPage) | 0);

  if (store.comments_count > 0) {
    {
      let index = store.comment_page_index;
      let d = 1;
      while (index > 1) {
        indices.add(index);
        index -= d;
        d *= 2;
      }
      indices.add(1);
    }
    {
      let index = store.comment_page_index;
      let d = 1;
      while (index < lastIndex) {
        indices.add(index);
        index += d;
        d *= 2;
      }
      indices.add(lastIndex);
    }
  }

  return Array.from(indices).sort();
});
const subLinkCommentPageIndices = computed(() => {
  if (!store.sub_pagination) {
    return [];
  }

  const count = store.sub_pagination.item_count;
  const item_per_page = store.sub_pagination.item_per_page;
  const page_index = store.sub_pagination.index;

  const indices: Set<number> = new Set();
  const lastIndex = Math.max(1, ((count + item_per_page - 1) / item_per_page) | 0);

  {
    let index = page_index;
    let d = 1;
    while (index > 1) {
      indices.add(index);
      index -= d;
      d *= 2;
    }
    indices.add(1);
  }
  {
    let index = page_index;
    let d = 1;
    while (index < lastIndex) {
      indices.add(index);
      index += d;
      d *= 2;
    }
    indices.add(lastIndex);
  }

  return Array.from(indices).sort();
});

function sub_pagination_jump(index: number) {
  if (store.comment_shows_reply) {
    store.loadCommentReply(store.comment_shows_reply, index);
  }
  if (store.comment_shows_context) {
    store.loadCommentContext(store.comment_shows_context, index);
  }
}

const { comment_showlist } = storeToRefs(store);

</script>

<template>
  <div v-if="store.page_loading">loading...</div>
  <div class="comment-view" :style="custom_style" v-else>
    <CommentForm></CommentForm>
    <nav class="pagination_nav">
      <button v-for="index in linkCommentPageIndices" @click="store.loadComment(index)" class="comment_page_btn"
        :data-isactive="store.comment_page_index == index">{{  index  }}</button>
    </nav>
    <nav v-if="store.sub_pagination" class="pagination_nav sub_pagination_nav">
      <button v-for="index in subLinkCommentPageIndices" class="comment_page_btn" @click="sub_pagination_jump(index)"
        :data-isactive="store.sub_pagination.index == index">{{  index  }}</button>
    </nav>
    <div class="post-list">
      <CommentPost v-for="comment in comment_showlist" :key="comment.comment_id" :comment="comment" :is_admin="props.is_admin"
        @begin-reply-clicked="on_begin_reply_clicked" @cancel-reply-clicked="on_cancel_reply_clicked"
        @show-replies-clicked="show_replies" @show-contexts-clicked="show_contexts"
        @mark-comment-clicked="mark_comment">
      </CommentPost>
    </div>
    <nav v-if="store.sub_pagination" class="pagination_nav sub_pagination_nav">
      <button v-for="index in subLinkCommentPageIndices" class="comment_page_btn" @click="sub_pagination_jump(index)"
        :data-isactive="store.sub_pagination.index == index">{{  index  }}</button>
    </nav>
    <nav class="pagination_nav">
      <button v-for="index in linkCommentPageIndices" @click="store.loadComment(index)" class="comment_page_btn"
        :data-isactive="store.comment_page_index == index">{{  index  }}</button>
    </nav>

    <p>Powered by <a target="_blank" rel="noopener noreferrer" :href="repository_url">Masacarri</a></p>
  </div>

</template>

<style scoped>
.comment-view {
  max-width: 36rem;
}

.comment_page_btn {
  border: 1px solid #ddd;
  color: rgb(13, 139, 97);
  width: 2em;
  height: 2em;
  margin: 0.5em;
  font-size: 1rem;
  transition: background-color 0.1s;
}

.comment_page_btn:hover {
  background-color: #ccc;
}

.comment_page_btn[data-isactive=true] {
  background-color: rgb(86, 153, 131);
  color: #fff;
}

.pagination_nav {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
}

.sub_pagination_nav .comment_page_btn {
  font-size: 0.8rem;
}
</style>
