const recommendations = [
  "Complete advanced plumbing micro-task to unlock premium gigs.",
  "Respond within 10 minutes to improve response-speed factor.",
  "Request endorsements from 3 repeat clients this week.",
];

const formula = [
  { label: "Completed jobs", value: "40%" },
  { label: "Payment reliability", value: "25%" },
  { label: "Customer reviews", value: "20%" },
  { label: "Response speed", value: "15%" },
];

export default function InsightsPage() {
  return (
    <div className="grid gap-6 lg:grid-cols-2">
      <section className="rounded-2xl bg-white p-6 shadow-sm">
        <h1 className="text-2xl font-bold text-slate-900">AI Recommendations</h1>
        <ul className="mt-4 space-y-3">
          {recommendations.map((recommendation) => (
            <li key={recommendation} className="rounded-xl bg-blue-50 px-4 py-3 text-sm text-blue-900">
              {recommendation}
            </li>
          ))}
        </ul>
      </section>

      <section className="rounded-2xl bg-white p-6 shadow-sm">
        <h2 className="text-2xl font-bold text-slate-900">Trust Score Model</h2>
        <p className="mt-2 text-sm text-slate-600">
          Transparent scoring helps workers grow financial identity over time.
        </p>
        <div className="mt-4 space-y-3">
          {formula.map((item) => (
            <div
              key={item.label}
              className="flex items-center justify-between rounded-xl bg-slate-50 px-4 py-3"
            >
              <p className="text-sm text-slate-700">{item.label}</p>
              <p className="text-sm font-bold text-slate-900">{item.value}</p>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}
