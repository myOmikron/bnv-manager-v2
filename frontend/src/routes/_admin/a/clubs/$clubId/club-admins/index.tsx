import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/_admin/a/clubs/$clubId/club-admins/')({
  component: () => <div>Hello /_admin/a/clubs/$clubId/club-admins!</div>,
})
