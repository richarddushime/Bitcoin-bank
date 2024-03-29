import VueRouter from 'vue-router';
import GetBalance from '../components/GetBalance.vue';
import TransactionHistory from '../components/TransactionHistory.vue';

const routes = [
  {
    path: '/',
    component: GetBalance,
  },
  {
    path: '/transactions',
    component: TransactionHistory,
  },
];

const router = new VueRouter({
    mode: 'history',
    routes,
  });

export default router;
