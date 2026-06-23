import { useState } from "react";
import type { Hash } from "viem";
import { deposit } from "../handlers/deposit";
import { withdraw } from "../handlers/withdraw";
import type { Note } from "../lib/prover";

function etherscanTx(hash: Hash) {
  return `https://sepolia.etherscan.io/tx/${hash}`;
}

function TxLink({ hash }: { hash: Hash }) {
  return (
    <a
      href={etherscanTx(hash)}
      target="_blank"
      rel="noreferrer"
      className="font-mono text-xs text-emerald-400 hover:text-emerald-300 underline break-all"
    >
      {hash}
    </a>
  );
}

export default function Home() {
  // deposit state
  const [depositing, setDepositing] = useState(false);
  const [note, setNote] = useState<Note | null>(null);
  const [depositHash, setDepositHash] = useState<Hash | null>(null);

  // withdraw state
  const [recipient, setRecipient] = useState("");
  const [withdrawing, setWithdrawing] = useState(false);
  const [withdrawHash, setWithdrawHash] = useState<Hash | null>(null);

  const [error, setError] = useState<string | null>(null);

  async function onDeposit() {
    setError(null);
    setDepositHash(null);
    setDepositing(true);
    try {
      const result = await deposit();
      setNote(result.note);
      setDepositHash(result.hash);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setDepositing(false);
    }
  }

  async function onWithdraw() {
    setError(null);
    setWithdrawHash(null);
    if (!/^0x[0-9a-fA-F]{40}$/.test(recipient)) {
      setError("Enter a valid recipient address (0x… 40 hex chars).");
      return;
    }
    setWithdrawing(true);
    try {
      const hash = await withdraw(recipient as `0x${string}`);
      setWithdrawHash(hash);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setWithdrawing(false);
    }
  }

  return (
    <div className="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col items-center px-6 py-12">
      <header className="w-full max-w-xl flex items-center justify-between mb-10">
        <span className="font-mono font-bold text-lg tracking-tight">zk-mixer</span>
        <span className="text-xs text-zinc-500 font-mono">Sepolia · 0.01 ETH</span>
      </header>

      <main className="w-full max-w-xl flex flex-col gap-6">
        {error && (
          <div className="rounded-lg border border-red-900 bg-red-950/50 px-4 py-3 text-sm text-red-300">
            {error}
          </div>
        )}

        {/* DEPOSIT */}
        <section className="rounded-2xl border border-zinc-800 bg-zinc-900/40 p-6 flex flex-col gap-4">
          <div>
            <h2 className="text-lg font-semibold">Deposit</h2>
            <p className="text-sm text-zinc-400">
              Generate a secret note and lock 0.01 ETH into the mixer.
            </p>
          </div>

          <button
            onClick={onDeposit}
            disabled={depositing}
            className="self-start px-6 py-2.5 rounded-xl bg-zinc-100 text-zinc-900 font-semibold text-sm hover:bg-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {depositing ? "Depositing…" : "Deposit 0.01 ETH"}
          </button>

          {note && (
            <div className="rounded-lg border border-amber-900 bg-amber-950/40 p-4 flex flex-col gap-2">
              <p className="text-xs font-semibold text-amber-300">
                Save this note — it's the only way to withdraw. Lose it, lose the funds.
              </p>
              <pre className="text-[11px] font-mono text-amber-100/90 whitespace-pre-wrap break-all">
                {JSON.stringify(note, null, 2)}
              </pre>
            </div>
          )}

          {depositHash && (
            <div className="text-sm text-zinc-400">
              Tx: <TxLink hash={depositHash} />
            </div>
          )}
        </section>

        {/* WITHDRAW */}
        <section className="rounded-2xl border border-zinc-800 bg-zinc-900/40 p-6 flex flex-col gap-4">
          <div>
            <h2 className="text-lg font-semibold">Withdraw</h2>
            <p className="text-sm text-zinc-400">
              Prove ownership of your saved note and send 0.01 ETH to any address.
            </p>
          </div>

          <input
            value={recipient}
            onChange={(e) => setRecipient(e.target.value.trim())}
            placeholder="Recipient address (0x…)"
            className="w-full rounded-xl border border-zinc-700 bg-zinc-950 px-4 py-2.5 text-sm font-mono text-zinc-100 placeholder:text-zinc-600 focus:outline-none focus:border-zinc-500"
          />

          <button
            onClick={onWithdraw}
            disabled={withdrawing}
            className="self-start px-6 py-2.5 rounded-xl border border-zinc-700 text-zinc-200 font-semibold text-sm hover:border-zinc-500 hover:text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {withdrawing ? "Proving & withdrawing…" : "Withdraw"}
          </button>

          {withdrawing && (
            <p className="text-xs text-zinc-500">
              Generating the zero-knowledge proof — this freezes the tab for a few seconds.
            </p>
          )}

          {withdrawHash && (
            <div className="text-sm text-zinc-400">
              Tx: <TxLink hash={withdrawHash} />
            </div>
          )}
        </section>
      </main>

      <footer className="mt-10 text-xs text-zinc-600 font-mono">
        BN254 · Groth16 · Poseidon
      </footer>
    </div>
  );
}
