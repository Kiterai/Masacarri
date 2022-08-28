import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'

import './assets/main.css'
import { marked } from 'marked';
import sanitizeHtml from 'sanitize-html';

const marked_renderer = new marked.Renderer();
marked_renderer.link = (href, title, text) => {
    const href_attr = href ? `href="${href}"` : "";
    const title_attr = title ? `title="${title}"` : "";
    return `<a ${href_attr} ${title_attr} target="_blank" rel="noopener noreferrer">${text}</a>`;
};
marked.setOptions({
    breaks: true,
    renderer: marked_renderer,
});
sanitizeHtml.defaults.disallowedTagsMode = 'recursiveEscape';
sanitizeHtml.defaults.allowedTags = sanitizeHtml.defaults.allowedTags.concat(['del']);

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.use(router)

app.mount('#app')
