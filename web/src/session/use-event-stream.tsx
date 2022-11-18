/*
 * Copyright (C) 2022 - This file is part of "japp".
 *
 * "japp" is free software: you can redistribute it and/or modify it under the
 *  terms of version 3 of the GNU Affero General Public License as published by the
 *  Free Software Foundation.
 *
 * "japp" is distributed in the hope that it will be useful, but WITHOUT ANY
 *  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 *  FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
 *   details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with japp.  If not, see http://www.gnu.org/licenses/.
 */

import { useSessionParams } from './use-session-params'
import { useRef } from 'react'

export const useEventStream = () => {
  const state = useSessionParams()
  const stream = useRef<EventSource>()
  const join = () => {
    return new Promise<void>(resolve => {
      stream.current = new EventSource(`/api/events/${state.sessionId}`)
      stream.current.addEventListener('open', () => {
        resolve()
      })
      stream.current.addEventListener('message', event => {
        console.log(event.data)
      })
      stream.current.addEventListener('error', err => {
        console.error(err)
      })
    })
  }

  const close = () => {
    stream.current?.close()
  }

  return [join, close]
}
