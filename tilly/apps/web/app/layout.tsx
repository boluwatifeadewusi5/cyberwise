import type { Metadata } from "next";
import "./globals.css";
import { Nav } from "./components/nav";

export const metadata: Metadata = {
  title: "TILLY | AI-Powered Opportunity Engine for Youth",
  description:
    "A skills-first gig and jobs marketplace where youth can prove ability without formal work history.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="h-full antialiased">
      <body className="min-h-full bg-slate-50 text-slate-900">
        <div className="min-h-screen bg-[radial-gradient(circle_at_top,#dbeafe_0%,#f8fafc_45%,#fefce8_100%)]">
          <Nav />
          <main className="mx-auto w-full max-w-md px-4 pb-24 pt-4">{children}</main>
        </div>
      </body>
    </html>
  );
}
