import Contract from '@redspot/patract/contract'
import BN from 'bn.js'
import { artifacts, network, patract } from 'redspot'
import { Signer } from 'redspot/types'

const { getContractFactory, getRandomSigner } = patract
const { api, getSigners } = network

export { expect } from './setup/chai'

const patchContractMethods = (contract: Contract): Contract => {
  patchMethods(contract.query)
  patchMethods(contract.tx)
  return contract
}

// It removes prefix from the function and adds only name of method like a function
// Erc20::token_name
// query["Erc20,tokenName"]
// query.tokenName()
const patchMethods = (object) => {
  for (const prop in object) {
    if (prop.includes(',')) {
      const selectors = prop.split(',')
      const method = selectors[selectors.length - 1]
      object[method] = object[prop]
    }
  }
}

export const setupContract = async (name, constructor, ...args) => {
  const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]))
  const signers = await getSigners()
  const defaultSigner = await getRandomSigner(signers[0], one.muln(10000))
  const alice = await getRandomSigner(signers[1], one.muln(10000))

  const contractFactory = await getContractFactory(name, defaultSigner)
  const contract = await contractFactory.deploy(constructor, ...args)
  const abi = artifacts.readArtifact(name)
  patchContractMethods(contract)

  return {
    defaultSigner,
    alice,
    accounts: [alice, await getRandomSigner(), await getRandomSigner()],
    contractFactory,
    contract,
    abi,
    one,
    query: contract.query,
    tx: contract.tx
  }
}

export const fromSigner = (contract: Contract, address: string): Contract => {
  return patchContractMethods(contract.connect(address))
}

export const bnArg = (value: number | string | number[] | Uint8Array | Buffer | BN, length = 32) => new BN(value, undefined, 'le').toArray('le', length)