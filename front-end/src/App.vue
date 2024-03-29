<template>
  <div id="app">
    <login-form v-if="!isLoggedIn" @login-success="handleLoginSuccess" />
    <div v-else>
      <userBalance v-if="user.role === 'user'" />
      <GetBalance v-if="user.role === 'admin'" />
      <a v-if="user.role === 'admin'" href="#">Welcome, Admin</a>  
    </div>
  </div>
  <!-- <TransactionHistory /> -->
  <aFooter />
  <modalPop />
</template>

<script>
import LoginForm from './components/LoginForm.vue';
import GetBalance from './components/GetBalance.vue';
import userBalance from './components/userBalance';
// import TransactionHistory from './components/TransactionHistory.vue';
import aFooter from './components/aFooter.vue';
import modalPop from './components/modalPop.vue';


export default {
  name: 'App',
  components: {
    LoginForm,
    userBalance,
    GetBalance,
    // TransactionHistory,
    aFooter, 
    modalPop,
  },
  data() {
    return {
      isLoggedIn: false,
      user: null,
      showTransactionDetailsPopup: false,
    };
  },
  methods: {
    handleLoginSuccess(user) {
      this.isLoggedIn = true;
      this.user = user;
    },
  },
  created() {
    // Check for pre-existing logins
    const storedUser = localStorage.getItem('user');
    if (storedUser) {
      this.user = JSON.parse(storedUser);
      this.isLoggedIn = true;
    }
  },
};
</script>

<style>
  #app {
    font-family: Arial, Helvetica, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #2c3e50;
    margin-top: 60px;
  }
</style>./components/modalPop.vue