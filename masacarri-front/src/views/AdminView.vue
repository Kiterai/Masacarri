<script setup lang="ts">
import { app_fetch_admin } from '@/utils';
import { ref } from 'vue';
import PagesForm from '../components/admin/PagesForm.vue';

const login_form = ref({
  user: '',
  password: '',
});
const login_form_info = ref<string | null>(null);
const is_logined = ref(false);

function login() {
  login_form_info.value = "in progress...";
  app_fetch_admin(`/api/login`, "POST", login_form.value)
    .then(res => {
      is_logined.value = true;
      login_form_info.value = null;
    }).catch(err => {
      login_form_info.value = "login error";
    });
}

function logout() {
  login_form_info.value = "in progress...";
  app_fetch_admin(`/api/logout`)
    .then(res => {
      is_logined.value = false;
      login_form_info.value = null;
    })
    .catch(err => {
      login_form_info.value = "logout error";
    });
}

</script>

<template>
  <div id="admin">
    <div v-if="is_logined">
      <nav>Successfully loginned</nav>

      <nav><button type="button" @click="logout">[logout]</button></nav>

      <PagesForm></PagesForm>

    </div>
    <div v-else>
      <nav>You must login</nav>
      <form>
        <p>user: <input v-model="login_form.user" type="text" name="user" /></p>
        <p>password: <input v-model="login_form.password" type="password" name="password" /></p>
        <p class="form_info" v-if="login_form_info">{{  login_form_info  }}</p>
        <button type="button" @click="login">[login]</button>
      </form>
    </div>
  </div>
</template>

<style>
#admin {
  padding: 1em;
}

.form_info {
  color: #F00;
}
</style>
