interface OkxWallet {
  on: (event: 'accountChanged', handler: (accounts: { address: string, publicKey: string, compressedPublicKey: string }) => void) => void
  sendBitcoin: (address: string, amount: number, options?: {
    feeRate: number
  }) => Promise<string>
  requestAccounts(): Promise<Array<string>>
  disconnect: () => void
  getBalance: () => Promise<{
    confirmed: number
    unconfirmed: number
    total: number
  }>
}

interface UnisatWallet {
  sendBitcoin: (address: string, satoshis: number, options?: {
    feeRate: number
  }) => Promise<string>
  on: (event: string, handler: (accounts: Array<string>) => void) => void
  removeListener(event: string, handler: (accounts: Array<string>) => void)
  requestAccounts(): Promise<Array<string>>
  getBalance: () => Promise<{
    confirmed: number
    unconfirmed: number
    total: number
  }>
}

declare global {
  var unisat: UnisatWallet
  var okxwallet: {
    bitcoin: OkxWallet
    bitcoinTestnet: OkxWallet
  }
}

export { };