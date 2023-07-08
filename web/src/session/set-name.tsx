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

import { useEffect, useState } from 'react'
import { Input } from '../common/input'
import { useBackend } from '../common/use-backend'
import { useSessionStore } from './session-state'
import { useSessionParams } from './use-session-params'
import { Spinner } from '../common/spinner'

export const SetName = () => {
  const { sessionId } = useSessionParams()
  const backend = useBackend()
  const [name, setName] = useState<string | undefined>('')
  const [nameValid, setNameValid] = useState<boolean>(false)
  const [loading, setLoading] = useState(false)
  const { setOwnName } = useSessionStore()
  useEffect(() => setNameValid(!!name?.trim()), [name])

  const click = () => {
    if (!nameValid) { return }
    setLoading(true)
    backend.put(undefined, `/session/${sessionId}/participant/${name}`)
      .text(() => setOwnName(name))
      .finally(() => setLoading(false))
  }

  return <>
    <Input
      name="name"
      label={<>Your Name</>}
      hint={<>What do your teammates call you?</>}
      icon={<i className="icons8">&#xf10d;</i>}
      errorMessage={!nameValid ? 'Please enter any name' : undefined}
      value={name}
      change={setName}
    />
    <button disabled={!name || loading} onClick={click}>
    {loading && <Spinner size={'inline'}/>}
    <i className="icons8">&#xf10e;</i>Join session
    </button>
  </>
}
