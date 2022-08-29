import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'

import './assets/main.css'
import { marked } from 'marked';
import sanitizeHtml from 'sanitize-html';
import { updateHeight } from './utils'

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

const app_elem = document.querySelector("#app");
if (app_elem) {
    const resizeObserver = new ResizeObserver(entries => {
        for (let entry of entries) {
            if (entry.target == app_elem) {
                updateHeight(entry.borderBoxSize[0].blockSize);
            }
        }
    });
    resizeObserver.observe(app_elem);
}
