# Learning NEXTJS REST API Project
Next.js is a powerful React framework that simplifies building server-side rendered (SSR) and statically generated (SSG) web applications. It provides a robust development environment with features like automatic code splitting, server-side rendering, and easy deployment, making it an ideal choice for building modern web applications with React.

## Getting Running

### Installation/Prerequisites
Make sure to have [Node.js](https://nodejs.org/en) installed on your machine. Clone or fork the repository locally to get started.

### Running Project
From this directory, run `npm install`. To start the server on port 8000, run `npm run dev`. 

### Language Resources
Run `npx create-next-app@latest` to get started with a next application.

### Language Project layout resources

- [Next Routing](https://nextjs.org/docs/app/building-your-application/routing/colocation)
- Next [App Router](https://nextjs.org/docs/app)

### Language Specifics
* [Main Site](https://nextjs.org/)
* Building an API using [Route Handlers](https://nextjs.org/docs/app/building-your-application/routing/route-handlers#url-query-parameters)
* Using [CORS with Next](https://nextjs.org/docs/app/building-your-application/routing/route-handlers#cors) (advanced)
* [Blog on using CORS](https://blog.logrocket.com/using-cors-next-js-handle-cross-origin-requests/) (advanced)

#### Summary

Here are some reasons to use Next: 
1. Efficient Development: Next.js offers a streamlined development experience with features like automatic code splitting and hot module replacement, reducing boilerplate and speeding up development.
2. Server-side Rendering (SSR): SSR in Next.js improves SEO by serving fully-rendered pages to search engine crawlers and enhances performance by reducing initial load times.
3. Static Site Generation (SSG): Next.js supports SSG, enabling pre-rendering of pages at build time, resulting in faster page loads and reduced server load.

However, there are also scenarios where Next.js may not be the best choice:
1. Simple Static Sites: For simple static sites without the need for SSR or complex data fetching, a simpler static site generator like Gatsby may be more appropriate.
2. Learning Curve: Next.js introduces additional complexity compared to basic React applications, which may require some learning time for developers unfamiliar with its concepts.
