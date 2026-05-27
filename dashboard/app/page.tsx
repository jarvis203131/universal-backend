export default function Home() {
  return (
    <div className="space-y-6">
      <h1 className="text-4xl font-bold tracking-tight">System Overview</h1>
      <div className="grid grid-cols-3 gap-6">
        <div className="p-6 bg-white border rounded-xl shadow-sm">
          <div className="text-sm font-medium text-slate-500">Active Tenants</div>
          <div className="text-3xl font-bold">12</div>
        </div>
        <div className="p-6 bg-white border rounded-xl shadow-sm">
          <div className="text-sm font-medium text-slate-500">Event Throughput</div>
          <div className="text-3xl font-bold">1.2k/sec</div>
        </div>
        <div className="p-6 bg-white border rounded-xl shadow-sm">
          <div className="text-sm font-medium text-slate-500">API Latency</div>
          <div className="text-3xl font-bold">14ms</div>
        </div>
      </div>
      <div className="p-12 bg-slate-900 text-white rounded-2xl text-center">
        <h2 className="text-2xl font-semibold mb-2">Universal Backend Engine Online</h2>
        <p className="text-slate-400">Precision. Excellence. Absolute Isolation.</p>
      </div>
    </div>
  )
}
