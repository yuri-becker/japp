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

import { useState } from 'react'
import { useBackend } from '../../common/use-backend'
import { CreateSessionRequest } from './create-session-request'
import { CreateSessionResponse } from './create-session-response'

export const useCreateSession = () => {
  const backend = useBackend()
  const [loading, setLoading] = useState(false)

  const createSession = (request: CreateSessionRequest) => new Promise<CreateSessionResponse>((resolve, reject) => {
    setLoading(true)
    backend
      .url('/session')
      .post(request)
      .json<CreateSessionResponse>()
      .then(createdSession => resolve(createdSession))
      .catch(reject)
      .finally(() => setLoading(false))
  })

  return [createSession, loading] as const
}
