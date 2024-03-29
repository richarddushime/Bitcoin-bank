'use client';

import React from 'react'

const TransferButton = () => {
  const spend = async (data: any) => {
    try {
      const req = await fetch("http://127.0.0.1:3000/bitcoinbank/spendfromwallet",
        {
          method: 'POST',
          headers: {
            'content-type': 'application/json',
          },
          body: JSON.stringify(data),
          cache: 'no-cache'
        },
      )
      const res = await req.json()
      console.log(`res: ${res}`)
    } catch (error) {
      console.log(error)
    }
  }
  spend({
    "dest_address":"bcrt1qg4u6746cf0geu3ysrdn2dmpn5vtm3ye2dmsmhw",
    "amount": 50000
  });

  return (
    <div>
      <button onClick={spend} className="shadow cursor-pointer bg-purple-500 hover:bg-purple-400 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded" type="button">
        Transfer
      </button>
    </div>
  )
}

export default TransferButton