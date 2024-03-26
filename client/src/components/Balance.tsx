'use client';

import React from 'react'

const Balance = async () => {
  const res = await fetch("http://127.0.0.1:3000/bitcoinbank/getbalancefromwallet")
  const balance = await res.json()
  console.log(balance)

  return (
    <div className='w-full my-1'>
        {balance}
      </div>
  )
}

export default Balance