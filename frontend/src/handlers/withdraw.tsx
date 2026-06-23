import { encodeEventTopics, parseAbiItem, type Hash } from "viem";
import { generateProof, type Note } from "../lib/prover";
import { MIXER_ADDRESS, DEPLOY_BLOCK, RELAYER_ADDRESS, RELAYER_URL } from "../lib/chains";

const depositEvent = parseAbiItem(
    "event Deposit(bytes32 indexed commitment, uint32 leafIndex, uint256 timestamp)",
);
const DEPOSIT_TOPIC0 = encodeEventTopics({ abi: [depositEvent] })[0]!;

const ETHERSCAN_API = "https://api.etherscan.io/v2/api";
const CHAIN_ID = 11155111;
const PAGE_SIZE = 1000;

type EtherscanLog = { topics: string[]; data: string };


async function fetchCommitments(): Promise<string[]> {
    const logs: EtherscanLog[] = [];

    for (let page = 1; ; page++) {
        const params = new URLSearchParams({
            chainid: String(CHAIN_ID),
            module: "logs",
            action: "getLogs",
            address: MIXER_ADDRESS,
            topic0: DEPOSIT_TOPIC0,
            fromBlock: DEPLOY_BLOCK,
            toBlock: "latest",
            page: String(page),
            offset: String(PAGE_SIZE),
            apikey: import.meta.env.VITE_ETHERSCAN_API_KEY,
        });

        const res = await fetch(`${ETHERSCAN_API}?${params}`);
        const json = await res.json();

        if (json.status !== "1") {
            if (json.message === "No records found") break;
            throw new Error(`Etherscan getLogs failed: ${json.message ?? "unknown error"}`);
        }

        const batch = json.result as EtherscanLog[];
        logs.push(...batch);
        if (batch.length < PAGE_SIZE) break;
    }

    return logs
        .map((log) => ({
            commitment: log.topics[1],
            leafIndex: Number(BigInt(log.data.slice(0, 66))),
        }))
        .sort((a, b) => a.leafIndex - b.leafIndex)
        .map((x) => x.commitment);
}

export async function withdraw(recipient: `0x${string}`): Promise<Hash> {
    const raw = localStorage.getItem("note");
    if (!raw) throw new Error("No saved note found — make a deposit first.");
    const note: Note = JSON.parse(raw);

    const leaves = await fetchCommitments();

    const { proof, publicInputs } = await generateProof(note, leaves, recipient, RELAYER_ADDRESS);

    const res = await fetch(`${RELAYER_URL}/relayer`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            proof,
            root: publicInputs.root,
            nullifierHash: publicInputs.nullifier_hash,
            recipient,
        }),
    });

    if (!res.ok) {
        throw new Error(`Relayer error: ${await res.text()}`);
    }

    const { tx_hash } = await res.json();
    return tx_hash as Hash;
}
