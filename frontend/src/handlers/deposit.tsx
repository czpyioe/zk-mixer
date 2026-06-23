import { parseEther, type Hash } from "viem";
import { generateNote, type Note } from "../lib/prover";
import { walletClient, publicClient, mixerAbi, MIXER_ADDRESS } from "../lib/chains";

const DENOMINATION = parseEther("0.01");

export async function deposit(): Promise<{ note: Note; hash: Hash }> {
    const note = await generateNote();

    localStorage.setItem("note", JSON.stringify(note));

    const [account] = await walletClient.requestAddresses();
    const { request } = await publicClient.simulateContract({
        account,
        address: MIXER_ADDRESS,
        abi: mixerAbi,
        functionName: "deposit",
        args: [note.commitment as `0x${string}`],
        value: DENOMINATION,
    });
    const hash = await walletClient.writeContract(request);
    await publicClient.waitForTransactionReceipt({ hash });

    return { note, hash };
}
