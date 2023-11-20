import { error } from '@sveltejs/kit';

async function send({ method, path, data, token, cookie }: { method: any, path: any, data?: any, token?: any, cookie?: any }) {
    const opts: any = { method, headers: {}, credentials: 'include' };

    if (data) {
        opts.headers = {
            "Content-Type": "application/json",
        };
        opts.body = JSON.stringify(data);
    }

    if (token) {
        opts.headers['Authorization'] = `Bearer ${token}`;
    }

    if (cookie) {
        opts.headers['Cookie'] = `myCookie=${cookie}`;
    }

    // console.log(path);
    // console.log(opts);

    const response = await fetch(path, opts);

    // response.headers.forEach((value, key) => console.log(`${key}: ${value}`));

    if (response.ok || response.status === 422) {
        // get the json of the response
        const result = await response.json();

        // add pagination header to the result object
        const paginationHeader = response.headers.get('pagination');
        if (paginationHeader) result.pagination = paginationHeader;

        return result ? result : {};
    }

    throw error(response.status);
}

export function get(path: any, token?: any) {
    return send({ method: 'GET', path, token });
}

export function del(path: any, token: any) {
    return send({ method: 'DELETE', path, token });
}

export function post(path: any, data: any, token?: any, cookie?: any) {
    return send({ method: 'POST', path, data, token, cookie });
}

export function put(path: any, data: any, token: any) {
    return send({ method: 'PUT', path, data, token });
}
