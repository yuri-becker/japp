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

import { useSessionStore } from '../state/session.state'
import { useSessionParams } from '../use-session-params'
import { useRef } from 'react'
import { EventType, SessionEvent } from './session-event'

type ServerReceivedEvent<T extends EventType> = Record<T, SessionEvent['payload']>

export const useEventStream = () => {
  const state = useSessionParams()
  const consumeSessionEvent = useSessionStore(state => state.consumeSessionEvent)
  const stream = useRef<EventSource>()
  const join = () => new Promise<void>(resolve => {
    stream.current = new EventSource(`/api/session/${state.sessionId}/events`)
    stream.current.addEventListener('open', () => {
      console.debug(`Opened event stream for session id ${state.sessionId}`)
      resolve()
    })
    stream.current.addEventListener('message', (event: MessageEvent<string>) => {
      console.debug(`Event received: ${event.data}`)
      const data = JSON.parse(event.data) as ServerReceivedEvent<EventType>
      const eventType = Object.keys(data)[0] as EventType
      if (!eventType) {
        console.error(`Got an illegal event: ${event.data}`)
        return
      }
      consumeSessionEvent({ type: eventType, payload: data[eventType] as any })
    })
    stream.current.addEventListener('error', err => {
      console.error(err)
    })
  })

  const close = () => {
    if (stream.current) {
      console.debug('Closing event stream')
      stream.current?.close()
    }
  }

  return [join, close]
}
