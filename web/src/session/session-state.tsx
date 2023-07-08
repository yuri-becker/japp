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

import create, { StateCreator } from 'zustand'

declare type Slices = SessionNameSlice & OwnNameSlice

interface SessionNameSlice {
  sessionName?: string
  setSessionName: (sessionName: SessionNameSlice['sessionName']) => void
}

const createSessionNameSlice: StateCreator<Slices, [], [], SessionNameSlice> = set => ({
  sessionName: undefined,
  setSessionName: sessionName => set(state => ({ sessionName }))
})

interface OwnNameSlice {
  ownName?: string
  setOwnName: (ownName: OwnNameSlice['ownName']) => void
}

const createOwnNameSlice: StateCreator<Slices, [], [], OwnNameSlice> = set => ({
  ownName: undefined,
  setOwnName: ownName => set(() => ({ ownName }))
})

export const useSessionStore = create<Slices>((...a) => ({
  ...createSessionNameSlice(...a),
  ...createOwnNameSlice(...a)
}))
