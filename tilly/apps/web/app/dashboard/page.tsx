const user = {
  name: "Amina Adeyemi",
  location: "Ikeja, Lagos",
  score: 82,
  completedJobs: 13,
  reliability: 95,
};

const badges = [
  "Plumbing Basics Verified",
  "On-time Task Completion",
  "5 Peer Endorsements",
];

export default function DashboardPage() {
  return (
    <div className="space-y-4">
      <section className="rounded-2xl bg-white p-5 shadow-sm">
        <p className="text-sm font-medium text-slate-500">Skill Passport</p>
        <h1 className="mt-2 text-2xl font-bold text-slate-900">{user.name}</h1>
        <p className="mt-1 text-sm text-slate-600">{user.location}</p>

        <div className="mt-5 grid gap-3">
          <div className="rounded-xl bg-slate-50 p-4">
            <p className="text-xs text-slate-500">HustleScore</p>
            <p className="text-2xl font-bold text-slate-900">{user.score}</p>
          </div>
          <div className="rounded-xl bg-slate-50 p-4">
            <p className="text-xs text-slate-500">Completed Jobs</p>
            <p className="text-2xl font-bold text-slate-900">{user.completedJobs}</p>
          </div>
          <div className="rounded-xl bg-slate-50 p-4">
            <p className="text-xs text-slate-500">Payment Reliability</p>
            <p className="text-2xl font-bold text-slate-900">{user.reliability}%</p>
          </div>
        </div>
      </section>

      <section className="rounded-2xl bg-white p-5 shadow-sm">
        <h2 className="text-base font-semibold text-slate-900">Verification Badges</h2>
        <ul className="mt-3 space-y-2">
          {badges.map((badge) => (
            <li key={badge} className="rounded-lg bg-emerald-50 px-3 py-2 text-sm text-emerald-800">
              {badge}
            </li>
          ))}
        </ul>
      </section>
    </div>
  );
}
