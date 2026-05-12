import Link from "next/link";
import { getApiHealth } from "@/lib/api";

const features = [
  "Conversational onboarding in local languages",
  "AI-scored skill assessments and endorsements",
  "Location-aware matching for nearby gigs",
  "Instant payout flows for completed work",
  "Trust score and financial identity growth",
];

export default async function Home() {
  const apiHealth = await getApiHealth();

  return (
    <div className="space-y-8">
      <section className="rounded-3xl bg-white/90 p-8 shadow-sm md:p-12">
        <p className="inline-block rounded-full bg-emerald-100 px-3 py-1 text-xs font-semibold tracking-wide text-emerald-700 uppercase">
          AI-Powered Opportunity Engine for Youth
        </p>
        <h1 className="mt-4 text-4xl font-bold tracking-tight text-slate-900 md:text-5xl">
          TILLY connects skilled youth to real gigs — no CV required.
        </h1>
        <p className="mt-4 max-w-3xl text-lg text-slate-600">
          Build a Skill Passport through practical assessments and completed jobs.
          Employers discover talent through semantic matching, not keywords.
        </p>
        <div className="mt-8 flex flex-wrap items-center gap-3">
          <Link
            href="/marketplace"
            className="rounded-full bg-slate-900 px-5 py-3 text-sm font-semibold text-white"
          >
            Explore Marketplace
          </Link>
          <Link
            href="/dashboard"
            className="rounded-full border border-slate-300 bg-white px-5 py-3 text-sm font-semibold text-slate-800"
          >
            Open Skill Passport
          </Link>
          <span className="rounded-full bg-slate-100 px-4 py-2 text-xs font-semibold text-slate-700">
            API status: {apiHealth}
          </span>
        </div>
      </section>

      <section className="grid gap-4 md:grid-cols-2">
        {features.map((feature) => (
          <article key={feature} className="rounded-2xl bg-white/80 p-5 shadow-sm">
            <p className="text-sm font-medium text-slate-700">{feature}</p>
          </article>
        ))}
      </section>
    </div>
  );
}
