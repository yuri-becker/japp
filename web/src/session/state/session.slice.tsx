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

import { produce } from 'immer'
import { P, match } from 'ts-pattern'
import { StateCreator } from 'zustand'
import { SessionEvent } from '../api/session-event'
import { SessionResponse } from '../api/session-response'
import { Slices } from './session.state'

export interface SessionSlice {
  session?: SessionResponse
  consumeSessionEvent: (sessionEvent: SessionEvent) => void
}

export const createSessionSlice: StateCreator<Slices, [], [], SessionSlice> = set => ({
  session: undefined,
  consumeSessionEvent: sessionEvent => set(baseState => produce(baseState, draftState => {
    match(sessionEvent)
      .with({ type: 'SessionInit', payload: P.select() }, payload => {
        draftState.session = payload.session
      })
      .with({ type: 'ParticipantJoined', payload: P.select() }, payload => {
        const index = draftState.session!.participants.findIndex(it => it.id === payload.id)
        if (index !== -1) {
          draftState.session!.participants[index].name = payload.name
        } else {
           draftState.session!.participants.push({ id: payload.id, name: payload.name })
        }
      })
      .exhaustive()
  }))
})
