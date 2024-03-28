<template>
    <div class="bitcoin-bank">
      <h2>Bitcoin Bank</h2>
      <p>Experience peace of mind knowing your funds are stored in both hot and cold wallets, keeping them safe from potential threats.</p>
      
      <p class="balance">Received BTC: 5000</p> 
    </div>
    <p>Welcome User1</p>
  </template>
  
  import 
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