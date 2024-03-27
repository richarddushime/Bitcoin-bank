<template>
  <div class="bitcoin-bank">
    <h2>Bitcoin Bank</h2>
    <form @submit.prevent="spendFromWallet" class="spend-form">
      <label for="destinationAddress" class="label">Destination Address:</label><br>
      <input type="text" id="destinationAddress" v-model="destinationAddress" required class="input" /><br>
      <label for="amount" class="label">Amount (in satoshis):</label> <br>
      <input type="number" id="amount" v-model.number="amount" required class="input" /><br>
      <button type="submit" class="spend-button">Spend</button><br>
    </form>
    <p v-if="transactionId" class="transaction-id">Transaction ID: {{ transactionId }}</p>
    <p v-if="error" class="error-message">{{ error }}</p>
    <p class="balance">Balance: {{ balance }}</p>
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
        // console.log(data); 
        try {
          const response = await fetch(
            ("http://localhost:3000/bitcoinbank/spendfromwallet"),
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
            console.error("Error spending from wallet:", await response.json());
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

<style scoped>
.bitcoin-bank {
  font-family: sans-serif;
  margin: 2rem auto;
  max-width: 400px;
  padding: 1rem;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.spend-form {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.label {
  font-weight: bold;
}

.input,
.spend-button {
  padding: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}

.spend-button {
  background-color: #4CAF50;
  color: white;
  cursor: pointer;
}

.spend-button:hover {
  background-color: #3e8e41;
}

.transaction-id,
.error-message,
.balance {
  margin-top: 0;
}

.transaction-id {
  color: green;
}

.error-message {
  color: red;
}

.balance {
  font-style: italic;
}
</style>