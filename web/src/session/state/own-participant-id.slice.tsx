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

import { StateCreator } from 'zustand'
import { Slices, useSessionStore } from './session.state'

export interface OwnParticipantIdSlice {
  ownParticipantId?: string
  setOwnParticipantId: (ownParticipantId: OwnParticipantIdSlice['ownParticipantId']) => void
}

export const createOwnParticipantIdSlice: StateCreator<Slices, [], [], OwnParticipantIdSlice> = set => ({
  ownParticipantId: undefined,
  setOwnParticipantId: ownParticipantId => set(() => ({ ownParticipantId }))
})

export const useOwnParticipant = () => useSessionStore(state => state.session?.participants.find(particiant => particiant.id === state.ownParticipantId))
