"use client"
import React, { useState, useEffect } from 'react'

export default function ExplorerPage() {
  const [table, setTable] = useState('users')
  const [data, setData] = useState([])
  const [filter, setFilter] = useState('')
  const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080'

  const fetchData = async () => {
    try {
      const res = await fetch(`${apiUrl}/api/v1/${table}?filter=${filter}`)
      const json = await res.json()
      setData(json)
    } catch (e) {
      console.error("Explorer error:", e)
    }
  }

  useEffect(() => { fetchData() }, [table, filter])

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Database Explorer</h1>
        <div className="flex gap-4">
          <input 
            className="border p-2 rounded bg-white" 
            placeholder="Table name..." 
            value={table} 
            onChange={e => setTable(e.target.value)} 
          />
          <input 
            className="border p-2 rounded bg-white" 
            placeholder="Filter (col=op.val)..." 
            value={filter} 
            onChange={e => setFilter(e.target.value)} 
          />
          <button onClick={fetchData} className="bg-slate-900 text-white px-4 py-2 rounded">Refresh</button>
        </div>
      </div>

      <div className="bg-white border rounded-xl overflow-auto max-h-[70vh]">
        <table className="w-full text-left border-collapse">
          <thead className="bg-slate-50 border-b sticky top-0">
            <tr>
              {data.length > 0 && Object.keys(data[0]).map(key => (
                <th key={key} className="p-4 font-semibold">{key}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {data.length === 0 ? (
              <tr><td className="p-8 text-center text-slate-400">No data for table {table}.</td></tr>
            ) : (
              data.map((row: any, i) => (
                <tr key={i} className="border-b last:border-0 hover:bg-slate-50">
                  {Object.values(row).map((val: any, j) => (
                    <td key={j} className="p-4 font-mono text-xs">{JSON.stringify(val)}</td>
                  ))}
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  )
}
