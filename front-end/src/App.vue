<template>
  <div id="app">
    <login-form v-if="!isLoggedIn" @login-success="handleLoginSuccess" />
    <div v-else>
      <userBalance v-if="user.role === 'user'" />
      <GetBalance v-if="user.role === 'admin'" />
      <a v-if="user.role === 'admin'" href="#">Welcome, Admin</a>  
    </div>
  </div>
  <aFooter />
</template>

<script>
import LoginForm from './components/LoginForm.vue';
import GetBalance from './components/GetBalance.vue';
import userBalance from './components/userBalance';
import aFooter from './components/aFooter.vue';

export default {
  name: 'App',
  components: {
    LoginForm,
    userBalance,
    GetBalance,
    aFooter, 
  },
  data() {
    return {
      isLoggedIn: false,
      user: null,
    };
  },
  methods: {
    handleLoginSuccess(user) {
      this.isLoggedIn = true;
      this.user = user;
    },
  },
  created() {
    // Check for pre-existing login (for example purposes)
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
</style>