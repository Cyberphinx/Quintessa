import type { RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ request }) => {
    const data = await request.json()

    return new Response('ok', {
        status: 200,
        headers: {
            'set-cookie': `theme=${data}; Path=/; HttpOnly`,
        },
    });
};
