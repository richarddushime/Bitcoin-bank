import { balance } from '@/utils'
import React from 'react'

const Account = () => {
  return (
    <>
      <h1>Account</h1>
      <div>
        {balance.balance}
      </div>
    </>
  )
}

export default Account