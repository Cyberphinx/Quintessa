import type { Handle } from "@sveltejs/kit";
import { env } from '$env/dynamic/private';

const plausible = 'https://plausible.io/js/script.js https://*.plausible.io https://plausible.io/api/event';
const supabase = `${env.SUPABASE_URL}`;
const backend = `${env.API_URI}`;
const frontend = `${env.URI}`;
const frontendDomain = frontend.toString().split('://')[1];
const frontendDns = `${env.DNS_URI}`;
const frontendDnsDomain = frontendDns.toString().split('://')[1];

const allowedOrigins = `https://${frontendDomain} https://www.${frontendDomain} https://${frontendDnsDomain} https://www.${frontendDnsDomain} ${backend}`;

const securityHeaders = {
    'Strict-Transport-Security': 'max-age=31536000',
    'X-Frame-Options': 'DENY',
    'X-Content-Type-Options': 'nosniff',
    'Referrer-Policy': 'strict-origin-when-cross-origin',
    'X-XSS-Protection': '1; mode=block',
    'Content-Security-Policy': `upgrade-insecure-requests; default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval' ${plausible}; script-src-elem 'self' 'unsafe-inline' 'unsafe-eval' ${plausible}; connect-src 'self' blob: ${supabase} ${plausible}; worker-src 'self' blob:; img-src 'self' data: blob: ${supabase}; style-src 'self' 'unsafe-inline'; font-src 'self'; base-uri 'self'; form-action 'self' ${allowedOrigins}; frame-ancestors 'self'; frame-src 'self'`
};

export const handle: Handle = async ({ event, resolve }) => {
    const jwt = event.cookies.get('jwt');
    event.locals.token = jwt ? jwt : null;

    const response = await resolve(event);

    // const setCookieValue = response.headers.get('set-cookie');
    // console.log(`set-cookie value: ${setCookieValue}`);

    // set security headers
    Object.entries(securityHeaders).forEach(
        ([header, value]) => response.headers.set(header, value)
    );

    // CORS
    // Get the Origin of the request
    const origin = event.url.origin;
    // If the Origin of the request is in the list of allowed origins, set the Access-Control-Allow-Origin header to that origin
    if (origin && allowedOrigins.includes(origin)) {
        response.headers.append('Access-Control-Allow-Origin', origin);
        response.headers.append('Access-Control-Allow-Credentials', 'true');
        response.headers.append('Access-Control-Allow-Methods', 'GET, POST');
        response.headers.append('Access-Control-Allow-Headers', 'Content-Type');
    }

    return response;
};

