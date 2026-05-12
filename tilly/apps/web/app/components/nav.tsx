import Link from "next/link";

const navItems = [
  { href: "/", label: "Home" },
  { href: "/dashboard", label: "Dashboard" },
  { href: "/marketplace", label: "Marketplace" },
  { href: "/wallet", label: "Wallet" },
  { href: "/insights", label: "AI Insights" },
];

export function Nav() {
  return (
    <nav className="mx-auto flex w-full max-w-6xl flex-wrap items-center justify-between gap-4 px-6 py-5">
      <div>
        <p className="text-lg font-bold text-slate-900">TILLY</p>
        <p className="text-xs text-slate-600">Your skills are your CV.</p>
      </div>
      <div className="flex flex-wrap items-center gap-2 rounded-full bg-white/80 p-2 shadow-sm">
        {navItems.map((item) => (
          <Link
            key={item.href}
            href={item.href}
            className="rounded-full px-4 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-100"
          >
            {item.label}
          </Link>
        ))}
      </div>
    </nav>
  );
}
