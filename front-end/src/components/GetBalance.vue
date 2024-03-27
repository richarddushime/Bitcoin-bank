<template>
    <div>
      <h2>Bitcoin Bank</h2>
      <form @submit.prevent="spendFromWallet">
        <label for="destinationAddress">Destination Address:</label><br>
        <input type="text" id="destinationAddress" v-model="destinationAddress" required /><br>
        <label for="amount">Amount (in satoshis):</label> <br>
        <input type="number" id="amount" v-model.number="amount" required /><br>
        <button type="submit">Spend</button><br>
      </form>
      <p v-if="transactionId">Transaction ID: {{ transactionId }}</p>
      <p>Balance: {{ balance }}</p>
    </div>
  </template>
  
  <script>
  export default {
    data() {
      return {
        destinationAddress: "",
        amount: 0,
        transactionId: null,
        balance: null,
      };
    },
    methods: {
      async spendFromWallet() {
        const data = {
          dest_address: this.destinationAddress,
          amount: this.amount,
        };
  
        try {
          const response = await fetch(
            "http://localhost:3000/bitcoinbank/spendfromwallet",
            {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              body: JSON.stringify(data),
              mode: 'no-cors'
            }
          );
  
          if (response.ok) {
            const transactionId = await response.json();
            this.transactionId = transactionId;
            // Clearing form fields after successful spend
            this.destinationAddress = "";
            this.amount = 0;
          } else {
            console.error("Error spending from wallet:", await response.text());
          }
        } catch (error) {
          console.error("Error spending from wallet:", error);
        }
      },
      async getBalanceFromWallet() {
        try {
          const response = await fetch(
            "http://localhost:3000/bitcoinbank/getbalancefromwallet"
          );
  
          if (response.ok) {
            const balance = await response.json();
            console.log(balance);
            this.balance = balance;
          } else {
            console.error("Error getting balance:", await response.text());
          }
        } catch (error) {
          console.error("Error getting balance:", error);
        }
      },
    },
    created() {
      this.getBalanceFromWallet();
    },
  };
  </script>
  