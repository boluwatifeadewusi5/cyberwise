const gigs = [
  {
    title: "Fix leaking kitchen sink",
    location: "2.1 km away · Surulere",
    payout: "₦18,000",
    fit: "94% semantic match",
  },
  {
    title: "Install shower piping in new apartment",
    location: "3.8 km away · Yaba",
    payout: "₦22,500",
    fit: "89% semantic match",
  },
  {
    title: "Emergency pipe replacement",
    location: "4.2 km away · Ikeja",
    payout: "₦30,000",
    fit: "86% semantic match",
  },
];

export default function MarketplacePage() {
  return (
    <div className="space-y-4">
      <section className="rounded-2xl bg-white p-5 shadow-sm">
        <h1 className="text-2xl font-bold text-slate-900">Nearby Opportunities</h1>
        <p className="mt-2 text-sm text-slate-600">
          Location-aware matching prioritizes gigs close to you.
        </p>
      </section>

      <section className="grid gap-3">
        {gigs.map((gig) => (
          <article key={gig.title} className="rounded-2xl bg-white p-5 shadow-sm">
            <p className="text-xs font-semibold text-emerald-700">{gig.fit}</p>
            <h2 className="mt-2 text-lg font-semibold text-slate-900">{gig.title}</h2>
            <p className="mt-1 text-sm text-slate-600">{gig.location}</p>
            <p className="mt-4 text-xl font-bold text-slate-900">{gig.payout}</p>
            <button className="mt-4 w-full rounded-xl bg-slate-900 px-4 py-2 text-sm font-semibold text-white">
              Accept Gig
            </button>
          </article>
        ))}
      </section>
    </div>
  );
}
