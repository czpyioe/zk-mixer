import init, {generate_note, prove} from "../wasm/prover";

let ready: Promise<unknown> | null = null;
function ensureReady() {
    if(!ready) ready = init();
    return ready;
}

export type Note = {
    secret: string;
    nullifier: string;
    commitment: string;
}

export async function generateNote(): Promise<Note> {
    await ensureReady();
    return JSON.parse(generate_note());
}

export type ProofResult = {
    proof: string;
    publicInputs: { 
        root: string; 
        nullifier_hash: string; 
        recipient_address: string 
    };
};

export async function generateProof(
    note: { 
        secret: string; 
        nullifier: string 
    },
    leaves: string[],
    recipient: string
): Promise<ProofResult> {
    await ensureReady();
    return JSON.parse(
        prove(
            note.secret, 
            note.nullifier, 
            JSON.stringify(leaves), 
            recipient
        )
    );
}