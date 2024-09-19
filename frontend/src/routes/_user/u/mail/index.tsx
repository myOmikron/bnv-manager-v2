import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/_user/u/mail/')({
  component: () => <div>Hello /mail/!</div>
})