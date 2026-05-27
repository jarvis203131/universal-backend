import './globals.css'
import type { Metadata } from 'next'
import { Inter } from 'next/font/google'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'Universal Backend Dashboard',
  description: 'Elite Administrative Control Panel',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <div className="flex min-h-screen">
          <aside className="w-64 bg-slate-900 text-white p-6 space-y-8">
            <div className="text-2xl font-bold tracking-tighter">UB DASHBOARD 🦾</div>
            <nav className="space-y-2">
              <a href="/auth" className="block p-2 hover:bg-slate-800 rounded">Auth Management</a>
              <a href="/explorer" className="block p-2 hover:bg-slate-800 rounded">Database Explorer</a>
              <a href="/realtime" className="block p-2 hover:bg-slate-800 rounded">Realtime Log</a>
            </nav>
          </aside>
          <main className="flex-1 p-8 bg-slate-50">
            {children}
          </main>
        </div>
      </body>
    </html>
  )
}
