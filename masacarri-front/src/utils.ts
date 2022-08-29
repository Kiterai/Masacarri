export function app_fetch<T>(path: string, method?: string, data?: T, is_admin?: boolean) {
    return fetch(import.meta.env.VITE_API_BASE_URL + path, {
        method: method ?? "GET",
        body: data ? JSON.stringify(data) : undefined,
        headers: {
            'Content-Type': 'application/json'
        },
        mode: import.meta.env.DEV ? 'cors' : undefined,
        credentials: is_admin ? (import.meta.env.DEV ? 'include' : 'same-origin') : undefined,
    }).then(res => {
        if (res.ok) {
            if (res.status === 204) {
                return res.headers;
            } else {
                return res.json();
            }
        } else {
            throw new Error();
        }
    });
}

export function app_fetch_admin<T>(path: string, method?: string, data?: T) {
    return app_fetch(path, method, data, true);
}

export const repository_url = import.meta.env.VITE_APP_REPOSITORY_URL;

export function updateHeight(h: number) {
    parent.postMessage(['masacarri-height', h], "*");
}
