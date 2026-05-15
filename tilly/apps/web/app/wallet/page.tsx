const transactions = [
  {
    id: "TXN-8834",
    gig: "Fix leaking kitchen sink",
    status: "Paid instantly",
    amount: "+₦18,000",
  },
  {
    id: "TXN-8671",
    gig: "Bathroom fittings setup",
    status: "Paid instantly",
    amount: "+₦14,500",
  },
  {
    id: "TXN-8442",
    gig: "Pipe maintenance",
    status: "Paid instantly",
    amount: "+₦11,000",
  },
];

export default function WalletPage() {
  return (
    <div className="space-y-6">
      <section className="rounded-2xl bg-white p-6 shadow-sm">
        <p className="text-sm text-slate-500">Squad Wallet Balance</p>
        <h1 className="mt-2 text-4xl font-bold text-slate-900">₦128,500</h1>
        <p className="mt-2 text-sm text-slate-600">
          Payments are disbursed immediately once tasks are marked complete.
        </p>
      </section>

      <section className="rounded-2xl bg-white p-6 shadow-sm">
        <h2 className="text-xl font-semibold text-slate-900">Transaction History</h2>
        <div className="mt-4 space-y-3">
          {transactions.map((txn) => (
            <article
              key={txn.id}
              className="flex items-center justify-between rounded-xl bg-slate-50 px-4 py-3"
            >
              <div>
                <p className="text-sm font-semibold text-slate-900">{txn.gig}</p>
                <p className="text-xs text-slate-500">
                  {txn.id} · {txn.status}
                </p>
              </div>
              <p className="text-sm font-bold text-emerald-700">{txn.amount}</p>
            </article>
          ))}
        </div>
      </section>
    </div>
  );
}
