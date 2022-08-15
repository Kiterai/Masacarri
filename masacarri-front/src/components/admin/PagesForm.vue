<script setup lang="ts">
import { app_fetch_admin } from '@/utils';
import { ref } from 'vue';

type PageData = {
    id: string,
    title: string,
    page_url: string,
    published: boolean
};

const pages = ref<PageData[]>([]);
function load_pages() {
    app_fetch_admin(`/api/pages`)
        .then(res => {
            pages.value = res;
        });
}
load_pages();

const page_form = ref({
    title: "",
    page_url: "",
    published: false,
});

const page_mod_form = ref({
    title: "",
    page_url: "",
    published: false,
});

const page_form_info = ref<string | null>(null);
const page_modify = ref<string | null>(null);

function add_page() {
    page_form_info.value = "in progress...";
    app_fetch_admin(`/api/pages`, "POST", page_form.value)
        .then((res: PageData) => {
            pages.value.push(res);
        }).catch(err => {
            page_form_info.value = "error";
        });
}

function begin_modify_page(id: string) {
    page_modify.value = id;
    const target_page = pages.value.find(page => page.id === id);
    if (target_page) {
        page_mod_form.value.title = target_page.title;
        page_mod_form.value.page_url = target_page.page_url;
        page_mod_form.value.published = target_page.published;
    }
}

function modify_page() {
    page_form_info.value = "in progress...";
    const id = page_modify.value;
    app_fetch_admin(`/api/pages/${id}`, "PATCH", page_mod_form.value)
        .then(res => {
            page_modify.value = null;
            const target_page = pages.value.find(page => page.id === id);
            if (target_page) {
                target_page.title = page_mod_form.value.title;
                target_page.page_url = page_mod_form.value.page_url;
                target_page.published = page_mod_form.value.published;
            }
        }).catch(err => {
            page_form_info.value = "error";
        });
}

function end_modify_page(id: string) {
    page_modify.value = null;
}

function delete_page(id: string) {
    page_form_info.value = "in progress...";
    app_fetch_admin(`/api/pages/${id}`, "DELETE", page_form.value)
        .then(res => {
            pages.value = pages.value.filter(page => page.id != id)
        }).catch(err => {
            page_form_info.value = "error";
        });
}

</script>

<template>
    <h2>Pages</h2>
    <table>
        <tbody>
            <tr>
                <th>ID</th>
                <th>Title</th>
                <th>URL</th>
                <th>state</th>
                <th></th>
            </tr>
            <tr v-for="page in pages">
                <td><a :href="`/pages/${page.id}`" target="_blank">{{ page.id }}</a></td>
                <td>{{ page.title }}</td>
                <td><a :href="page.page_url" target="_blank">{{ page.page_url }}</a></td>
                <td>{{ page.published ? 'published' : 'private' }}</td>
                <td v-if="!page_modify">
                    <button @click="begin_modify_page(page.id)">[Modify]</button>
                    /
                    <button @click="delete_page(page.id)">[Delete]</button>
                </td>
                <td v-else-if="page_modify == page.id">
                    <button @click="end_modify_page(page.id)">[Cancel Modify]</button>
                </td>
            </tr>
        </tbody>
    </table>
    <div v-if="page_modify">
        <h3>Modify Page</h3>
        <form>
            <p>Title: <input type="text" v-model="page_mod_form.title" /></p>
            <p>URL: <input type="text" v-model="page_mod_form.page_url" /></p>
            <p>published: <input type="checkbox" v-model="page_mod_form.published" /></p>
            <button type="button" @click="modify_page">[Modify Page]</button>
        </form>
    </div>
    <div v-else>
        <h3>Add Page</h3>
        <form>
            <p>Title: <input type="text" v-model="page_form.title" /></p>
            <p>URL: <input type="text" v-model="page_form.page_url" /></p>
            <p>published: <input type="checkbox" v-model="page_form.published" /></p>
            <button type="button" @click="add_page">[Add Page]</button>
        </form>
    </div>
</template>

<style scoped>
td {
    border-right: 1px solid;
    padding: 0.3em;
}
</style>
