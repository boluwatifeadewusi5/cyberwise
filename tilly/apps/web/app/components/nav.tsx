"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

const navItems = [
  { href: "/", label: "Home" },
  { href: "/dashboard", label: "Profile" },
  { href: "/marketplace", label: "Jobs" },
  { href: "/wallet", label: "Wallet" },
  { href: "/insights", label: "Insights" },
];

export function Nav() {
  const pathname = usePathname();

  return (
    <>
      <header className="mx-auto w-full max-w-md px-4 pt-5">
        <div className="rounded-2xl bg-white/90 px-4 py-3 shadow-sm">
          <p className="text-base font-bold text-slate-900">TILLY</p>
          <p className="text-xs text-slate-600">Your skills are your CV.</p>
        </div>
      </header>

      <nav className="fixed inset-x-0 bottom-0 z-50 mx-auto w-full max-w-md border-t border-slate-200 bg-white px-2 py-2">
        <ul className="grid grid-cols-5 gap-1">
          {navItems.map((item) => {
            const isActive = pathname === item.href;
            return (
              <li key={item.href}>
                <Link
                  href={item.href}
                  className={`block rounded-xl px-2 py-2 text-center text-[11px] font-semibold transition ${
                    isActive
                      ? "bg-slate-900 text-white"
                      : "bg-slate-100 text-slate-700"
                  }`}
                >
                  {item.label}
                </Link>
              </li>
            );
          })}
        </ul>
      </nav>
    </>
  );
}
