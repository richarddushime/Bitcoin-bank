  <template>
    <div class="bitcoin-bank">
      <h2>Bitcoin Bank</h2>
      <p>Experience peace of mind knowing your funds are stored in both hot and cold wallets, keeping them safe from potential threats.</p>
      <p class="balance"> Current Balance: <strong> {{ balance || 0 }}</strong></p>

      <p>
        <button @click="showTransactionDetailsPopup = true">
          View Transaction History
        </button>
      </p>
      <modal v-if="showTransactionDetailsPopup" @close="showTransactionDetailsPopup = false">
        <p v-if="transactionId" class="transaction-id">
          <strong>Transaction Details</strong> <br>
          TxID: {{ transactionId.txid }} <br>
          Bank Balance: {{ transactionId.bank_balance }} <br>
          Witness Hash: {{ transactionId.witness_hash }} <br>
          Version: {{ transactionId.version }} <br>
          Locktime: {{ transactionId.locktime }} <br>
        </p>
      </modal>

      <p>Ready to Spend ? Fill the Form</p>
      <p v-if="error" class="error-message">{{ error }}</p>
      <form @submit.prevent="spendFromWallet" class="spend-form">
        <label for="destinationAddress" class="label">Destination Address:</label>
        <input type="text" id="destinationAddress" v-model="destinationAddress" required class="input" /><br>
        <label for="amount" class="label">Amount (in satoshis):</label>
        <input type="number" min="1" id="amount" v-model.number="amount" required class="input" /><br>
        <button type="submit" class="spend-button">Spend</button><br>
      </form>
      <p v-if="transactionId" class="transaction-message">
        <strong>Transaction Successful!</strong><br>
        TxID: {{ transactionId.txid }} <br>
        Bank Balance: {{ transactionId.bank_balance }}
      </p>
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
        error: "",
        showTransactionDetailsPopup: false,
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
            }
          );

          if (response.ok) {
            try {
              const transactionId = await response.json();
              this.transactionId = transactionId;
              this.error = ""; 
              this.destinationAddress = "";
              this.amount = 0;
            } catch (error) {
              console.error("Error parsing JSON response:", error);
              this.error = "An error occurred while processing transaction.";
            }
          } else {
            const errorText = await response.text();
            console.error("Error spending from wallet:", await response.text());
            if (response.status === 511) {
              this.error = "Insufficient funds.";
            } else {
              this.error = `Server error: ${errorText}`;
            }
          }

        } catch (error) {
          console.error("Error spending from wallet:", error);
          this.error = "An error occurred while communicating with the server.";
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

label {
  font-weight: bold;
  float: left;
  display: block;
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