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

import { create } from 'zustand'
import { SessionSlice, createSessionSlice } from './session.slice'
import { OwnParticipantIdSlice, createOwnParticipantIdSlice } from './own-participant-id.slice'

export declare type Slices = OwnParticipantIdSlice & SessionSlice

export const useSessionStore = create<Slices>((...a) => ({
  ...createSessionSlice(...a),
  ...createOwnParticipantIdSlice(...a)
}))

export const useSessionName = () => useSessionStore(store => store.session?.name)
export const useOwnParticipant = () => useSessionStore(state => state.session?.participants.find(particiant => particiant.id === state.ownParticipantId))
