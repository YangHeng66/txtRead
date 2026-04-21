import { createRouter, createWebHashHistory } from 'vue-router'

export default createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/library' },
    { path: '/library', component: () => import('./views/LibraryView.vue') },
    { path: '/reader/:id', component: () => import('./views/ReaderView.vue'), props: true },
    { path: '/search', component: () => import('./views/SearchView.vue') },
  ],
})
