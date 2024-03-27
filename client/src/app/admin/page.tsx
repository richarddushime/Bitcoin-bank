'use client';

import React, { useEffect, useState } from 'react'
// import Balance from '@/components/Balance'
import TransferForm from '@/components/TransferForm'

const Admin = () => {
  const [balance, setBalance] = useState<number>();
  const getBalance = async () => {
    const res = await fetch("http://127.0.0.1:3000/bitcoinbank/getbalancefromwallet", {
      method: "GET",
      headers: {
        'content-type': 'application/json',
      }
    })
    const data = await res.json();
    console.log(data)
    setBalance(data)
  }

  useEffect(() => {
    getBalance();
  },[])

  return (
    <>
      <h1>Admin Account</h1>
      <div className='w-full my-1'>
        {balance}
      </div>
      <TransferForm/>
    </>
  )
}

export default Admin