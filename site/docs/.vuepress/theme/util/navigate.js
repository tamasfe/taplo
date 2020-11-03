/**
 * 导航
 *
 * @param url 跳转的网址
 * @param router 路由管理器
 * @param route 当前页面路由
 */
const navigate = (url, router, route) => {
    if (url)
        if (url.startsWith("/")) {
            // Inner absolute path
            if (route.path !== url)
                void router.push(url);
        }
        else if (url.startsWith("http://") ||
            url.startsWith("https://") ||
            url.startsWith("mailto:")) {
            // Outter url
            if (window)
                window.open(url);
        }
        else {
            // Inner relative path
            const base = route.path.slice(0, route.path.lastIndexOf("/"));
            void router.push(`${base}/${url}`);
        }
};
export default navigate;
