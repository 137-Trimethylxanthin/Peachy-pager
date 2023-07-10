/** @type {import('@sveltejs/kit').Handle} */
export const handle = async ({ event, resolve }) => {

    console.log('Server Hook');
    const theme = event.cookies.get("setTheme");
    console.log(theme);
    
    const response = await resolve(event, {
        transformPageChunk: ({ html }) =>
            html.replace('data-theme=""', `data-theme="${theme}"`),
    });

    return response;
}