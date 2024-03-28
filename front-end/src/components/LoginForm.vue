<template>
  <div class="login-form">
    <div class="logo-container">
      <img src="../assets/logo.png" alt="Logo" class="logo">
      <h2 class="login-header">Welcome Back!</h2>
    </div>
    <form @submit.prevent="login">
      <div class="form-group">
        <label for="username">Username:</label>
        <input type="text" id="username" v-model="username" required class="form-control">
      </div>
      <div class="form-group">
        <label for="password">Password:</label>
        <input type="password" id="password" v-model="password" required class="form-control">
      </div>
      <button type="submit" class="login-button">Login</button>
      <p>Your Trusted Bitcoin Vault</p>
      <p v-if="error" class="error-message">{{ error }}</p>
    </form>
  </div>
</template>

  
  <script>
  import users from '../data/users';
  
  export default {
    data() {
      return {
        username: "",
        password: "",
        error: null,
      };
    },
    methods: {
      async login() {
        const user = users.find(
          (u) => u.username === this.username && u.password === this.password
        );
  
        if (user) {
          this.$emit('login-success', user); // Emit event with user data
          this.username = "";
          this.password = "";
        } else {
          this.error = "Invalid username or password";
        }
      },
    },
  };
  </script>
  
  <style scoped>
    /* Scoped to the login-form component */
    .login-form {
      width: 300px;
      margin: 50px auto;
      border-radius: 10px;
      background-color: #f5f5f5;
      padding: 30px;
      box-shadow: 0px 2px 5px rgba(0, 0, 0, 0.1);
    }

    .logo-container {
      text-align: center;
      margin-bottom: 20px;
    }

    .logo {
      width: 100px;
      height: auto;
    }

    .login-header {
      font-weight: bold;
      margin-bottom: 15px;
    }

    .form-group {
      margin-bottom: 15px;
    }

    label {
      font-weight: bold;
      display: block;
      margin-bottom: 5px;
      float: left;
    }

    input[type="text"],
    input[type="password"] {
      width: 100%;
      padding: 10px;
      border: 1px solid #ccc;
      border-radius: 5px;
    }

    .login-button {
      background-color: #007bff;
      color: #fff;
      padding: 10px 20px;
      border: none;
      border-radius: 5px;
      cursor: pointer;
    }

    .error-message {
      color: red;
      font-size: 0.9em;
      margin-top: 10px;
    }

  </style>
  