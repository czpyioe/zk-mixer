import { createPublicClient, createWalletClient, custom, http, parseAbi } from "viem";
import { sepolia } from "viem/chains";


export const publicClient = createPublicClient({
    chain: sepolia,
    transport: http(import.meta.env.VITE_SEPOLIA_RPC_URL)
});



export const walletClient = createWalletClient({
    chain: sepolia,
    transport: custom(window.ethereum!)
})


export const MIXER_ADDRESS = import.meta.env.VITE_CONTRACT_ADDRESS;


export const DEPLOY_BLOCK = import.meta.env.VITE_DEPLOY_BLOCK;

export const mixerAbi = parseAbi([
    "function deposit(bytes32 commitment) payable",
    "function withdraw(bytes calldata proof, bytes32 root, bytes32 nullifierHash, address recipient)",
    "event Deposit(bytes32 indexed commitment, uint32 leafIndex, uint256 timestamp)",
]);