"use client"
import React, { useState, useEffect } from 'react'

export default function RealtimePage() {
  const [logs, setLogs] = useState<{id: string, type: string, payload: any}[]>([])
  const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080'

  useEffect(() => {
    const ws = new WebSocket(`${apiUrl.replace('http', 'ws')}/realtime`)
    
    ws.onmessage = (event) => {
      const data = JSON.parse(event.data)
      setLogs(prev => [{id: Date.now().toString(), type: data.event_type, payload: data.payload}, ...prev].slice(0, 50))
    }

    return () => ws.close()
  }, [])

  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Realtime Connection Log</h1>
      <div className="bg-slate-900 text-green-400 p-6 rounded-xl font-mono text-sm h-[70vh] overflow-y-auto space-y-2">
        {logs.length === 0 ? (
          <div className="text-slate-500 italic">Waiting for system events...</div>
        ) : (
          logs.map(log => (
            <div key={log.id} className="border-l-2 border-green-800 pl-4 py-1">
              <span className="text-slate-500">[{new Date().toLocaleTimeString()}]</span>{' '}
              <span className="font-bold"> {log.type}</span>: {JSON.stringify(log.payload)}
            </div>
          ))
        )}
      </div>
    </div>
  )
}
