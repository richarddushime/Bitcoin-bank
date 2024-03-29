<template>
    <div class="bitcoin-bank">
      <h2>Bitcoin Bank</h2>
      <p>Experience peace of mind knowing your funds are stored in both hot and cold wallets, keeping them safe from potential threats.</p>
      <p>Sending Address : bcrt1qdxsdrpd6np9ma6hp758779fc4mn8t0lgpfuyfx </p>
      <p class="balance">Received BTC: (sats) </p> 
    </div>
    <p><strong>Welcome User1</strong></p>
  </template>
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