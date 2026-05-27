"use client"
import React, { useState, useEffect } from 'react'

export default function AuthPage() {
  const [users, setUsers] = useState([])
  const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080'

  useEffect(() => {
    // Mocking fetch for structure as we don't have a dedicated /users endpoint yet, 
    // but this would hit the generic CRUD for the 'users' table.
    fetch(`${apiUrl}/api/v1/users`)
      .then(res => res.json())
      .then(data => setUsers(data))
      .catch(err => console.error("Auth fetch error:", err))
  }, [])

  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Auth Management</h1>
      <div className="bg-white border rounded-xl overflow-hidden">
        <table className="w-full text-left border-collapse">
          <thead className="bg-slate-50 border-b">
            <tr>
              <th className="p-4 font-semibold">User ID</th>
              <th className="p-4 font-semibold">Email</th>
              <th className="p-4 font-semibold">Project ID</th>
              <th className="p-4 font-semibold">Role</th>
            </tr>
          </thead>
          <tbody>
            {users.length === 0 ? (
              <tr><td colSpan="4" className="p-8 text-center text-slate-400">No users found.</td></tr>
            ) : (
              users.map((u: any) => (
                <tr key={u.id} className="border-b last:border-0 hover:bg-slate-50">
                  <td className="p-4 font-mono text-xs">{u.id}</td>
                  <td className="p-4">{u.email}</td>
                  <td className="p-4 font-mono text-xs">{u.project_id}</td>
                  <td className="p-4"><span className="px-2 py-1 bg-slate-100 rounded text-xs">Admin</span></td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  )
}
