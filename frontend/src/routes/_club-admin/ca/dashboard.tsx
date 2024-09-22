import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/_club-admin/ca/dashboard')({
  component: () => <div>Hello /_club-admin/ca/dashboard!</div>
})