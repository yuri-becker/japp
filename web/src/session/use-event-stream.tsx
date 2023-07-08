/*
 * Copyright (C) 2023 - This file is part of "JAPP".
 * "JAPP" is free software: you can redistribute it and/or modify it under the
 * terms of version 3 of the GNU Affero General Public License as published by the
 * Free Software Foundation.
 * "JAPP" is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
 * details.
 * You should have received a copy of the GNU Affero General Public License
 * long with JAPP.  If not, see http://www.gnu.org/licenses/.
 */

import { useSessionParams } from './use-session-params'
import { useRef } from 'react'

export const useEventStream = () => {
  const state = useSessionParams()
  const stream = useRef<EventSource>()
  const join = () => {
    return new Promise<void>(resolve => {
      stream.current = new EventSource(`/api/session/${state.sessionId}/events`)
      stream.current.addEventListener('open', () => {
        console.debug(`Opened event stream for session id ${state.sessionId}`)
        resolve()
      })
      stream.current.addEventListener('message', event => {
        console.debug(`Event received: ${event.data}`)
      })
      stream.current.addEventListener('error', err => {
        console.error(err)
      })
    })
  }

  const close = () => {
    if (stream.current) {
      console.debug('Closing event stream')
      stream.current?.close()
    }
  }

  return [join, close]
}
