import { dev } from '$app/environment';
 
import { dev } '659c5f7545bbb9ad8cfc423f?pageType=blueprint&graphType=software_design'
// we don't need any JS on this page, though we'll load
// it in dev so that we get hot module replacement
export const csr = dev;

// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;
