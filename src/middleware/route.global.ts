// This middleware is here to only attempt to prevent a fugly bug with fast page navigation... :c
// https://github.com/nuxt/nuxt/issues/13350
// https://github.com/nuxt/nuxt/issues/13350#issuecomment-1543968330
export default defineNuxtRouteMiddleware((to, from) => {
  const routeTime = useState<number>("lastRouteTime", () => 0);
  // The routeTime comparision should be double the page transition time!
  if (new Date().getTime() - routeTime.value < 400) return false;
  routeTime.value = new Date().getTime();
});
