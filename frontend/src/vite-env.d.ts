/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_CONTRACT_ADDRESS: `0x${string}`;
  readonly VITE_SEPOLIA_RPC_URL: string;
  readonly VITE_ETHERSCAN_API_KEY: string;
  readonly VITE_DEPLOY_BLOCK: string;
  readonly VITE_RELAYER_ADDRESS: `0x${string}`;
  readonly VITE_RELAYER_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

interface Window {
  ethereum?: import("viem").EIP1193Provider;
}
