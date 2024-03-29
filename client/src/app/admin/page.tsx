'use client';

import React, { useEffect, useState } from 'react'
const initialFormData = {
  "txid":"",
  "bank_balance":null,
  "witness_hash":"",
  "version":null,
  "locktime":null
}

const Admin = () => {
  const [balance, setBalance] = useState<number>();
  const [address, setAddress] = useState("")
  const [amount, setAmount] = useState("")
  const [formResponse, setFormResponse] = useState(initialFormData)

  const getBalance = async () => {
    const res = await fetch("http://127.0.0.1:3000/bitcoinbank/getbalancefromwallet", {
      cache: 'no-store',
      method: "GET",
      headers: {
        'content-type': 'application/json',
      }
    })
    const data = await res.json();
    // console.log(data)
    setBalance(data)
  }

  const handleSpend = async () => {
    try {
      const req = await fetch("http://127.0.0.1:3000/bitcoinbank/spendfromwallet",
        {
          method: 'POST',
          headers: {
            'content-type': 'application/json',
          },
          body: JSON.stringify({dest_address: address, amount: parseInt(amount)}),
          cache: 'no-cache'
        },
      )
      const res = await req.json()
      console.log(res)
      setFormResponse(res);
      getBalance()
    } catch (error) {
      console.log(error)
    }
  }

  // handleSpend({
  //   "dest_address":"bcrt1qg4u6746cf0geu3ysrdn2dmpn5vtm3ye2dmsmhw",
  //   "amount": 50000
  // });

  useEffect(() => {
    getBalance();
  },[])

  return (
    <>
      <div className='w-full'>
        <div className='w-full'>
          <h1 className="mt-4 ml-40">Admin Account</h1>
          <div className="w-full ml-40">
            {balance}
          </div>
        </div>
        <div>
          <form className="w-full max-w-sm">
            <div className="md:flex md:items-center mb-6">
              <div className="md:w-1/3">
                <label className="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" htmlFor="destination-address">
                  Destination Address
                </label>
              </div>
              <div className="md:w-2/3">
                <input value={address} onChange={(event) => {
                  setAddress(event.target.value)
                }} className="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" id="destination-address" type="text"/>
              </div>
            </div>
            <div className="md:flex md:items-center mb-6">
              <div className="md:w-1/3">
                <label className="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" htmlFor="amount">
                  Amount
                </label>
              </div>
              <div className="md:w-2/3">
                <input value={amount} onChange={(event) => {
                  setAmount(event.target.value)
                }} className="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" id="amount" type="number" placeholder="50000"/>
              </div>
            </div>
            <div className="md:flex md:items-center">
              <div className="md:w-1/3"></div>
              <div className="md:w-2/3">
              <button onClick={handleSpend} className="shadow cursor-pointer bg-purple-500 hover:bg-purple-400 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded" type="button">
                Transfer
              </button>
              </div>
            </div>
          </form>
          <div className='my-4 ml-20'>
              <p>TransactionID: {formResponse.txid}</p>
              <p>Balance: {formResponse.bank_balance} </p>
              <p>LockTime: {formResponse.locktime}</p>
          </div>
        </div>
      </div>
    </>
  )
}

export default Admin