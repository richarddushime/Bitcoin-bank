<template>
  <img alt="Vue logo" src="./assets/BitcoinBank.png">
  <GetBalance><p>Balance: {{ balance }}</p>
</GetBalance>
<HelloWorld msg="Bitcoin Bank "/>

</template>

<script>
import HelloWorld from './components/HelloWorld.vue';
import GetBalance from './components/GetBalance.vue';

export default {
  name: 'App',
  components: {
    GetBalance,
    HelloWorld,
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
