export default function Home() {
  return (
    <div className="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col">
      <header className="border-b border-zinc-800 px-6 py-4 flex items-center justify-between">
        <span className="font-mono font-bold text-lg tracking-tight">zk-mixer</span>
        <span className="text-xs text-zinc-500 font-mono">BN254 · Groth16 · Poseidon</span>
      </header>

      <main className="flex-1 flex flex-col items-center justify-center px-6 gap-12">
        <div className="flex flex-col items-center gap-4 text-center max-w-lg">
          <div className="w-16 h-16 rounded-2xl bg-zinc-800 flex items-center justify-center">
            <svg
              className="w-8 h-8 text-zinc-300"
              fill="none"
              stroke="currentColor"
              strokeWidth={1.5}
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z"
              />
            </svg>
          </div>
          <h1 className="text-4xl font-bold tracking-tight">zk-mixer</h1>
          <p className="text-zinc-400 text-base leading-relaxed">
            Private ETH transfers using zero-knowledge proofs.
            Deposit, break the on-chain link, withdraw anonymously.
          </p>
        </div>

        <div className="flex gap-4">
          <button
            className="px-8 py-3 rounded-xl bg-zinc-100 text-zinc-900 font-semibold text-sm hover:bg-white transition-colors cursor-not-allowed opacity-60"
            disabled
          >
            Deposit
          </button>
          <button
            className="px-8 py-3 rounded-xl border border-zinc-700 text-zinc-300 font-semibold text-sm hover:border-zinc-500 hover:text-zinc-100 transition-colors cursor-not-allowed opacity-60"
            disabled
          >
            Withdraw
          </button>
        </div>

        <div className="grid grid-cols-3 gap-px bg-zinc-800 rounded-2xl overflow-hidden max-w-lg w-full">
          {[
            { label: 'Denomination', value: '0.1 ETH' },
            { label: 'Anonymity set', value: '2²⁰ leaves' },
            { label: 'Proof system', value: 'Groth16' },
          ].map(({ label, value }) => (
            <div key={label} className="bg-zinc-950 px-5 py-4 flex flex-col gap-1">
              <span className="text-xs text-zinc-500 font-mono">{label}</span>
              <span className="text-sm font-semibold text-zinc-200">{value}</span>
            </div>
          ))}
        </div>
      </main>

      <footer className="border-t border-zinc-800 px-6 py-4 text-center">
        <span className="text-xs text-zinc-600 font-mono">work in progress</span>
      </footer>
    </div>
  )
}
