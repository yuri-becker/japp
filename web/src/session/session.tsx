/*
 * Copyright (C) 2022 - This file is part of "JAPP".
 *
 * "JAPP" is free software: you can redistribute it and/or modify it under the
 *  terms of version 3 of the GNU Affero General Public License as published by the
 *  Free Software Foundation.
 *
 * "JAPP" is distributed in the hope that it will be useful, but WITHOUT ANY
 *  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 *  FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
 *   details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with JAPP.  If not, see http://www.gnu.org/licenses/.
 */

import React, { useEffect, useState } from 'react'
import { useBackend } from '../common/use-backend'
import { useSessionStore } from './session-state'
import { SetName } from './set-name'
import { useSessionParams } from './use-session-params'
import { Page } from '../common/page'
import Session from './session'
import { SessionView } from './session-view'
import { Spinner } from '../common/spinner'

export default () => {
  const { sessionId, secret } = useSessionParams()
  const backend = useBackend()
  const { sessionName, setSessionName, ownName, setOwnName } = useSessionStore()
  const [credentialsValid, setCredentialsValid] = useState<boolean | undefined>()

  useEffect(() => {
    if (!sessionId || !secret) {
      return
    }
    backend.post(undefined, `/session/${sessionId}/participant?secret=${secret}`)
      .notFound(() => setCredentialsValid(false))
      .unauthorized(() => setCredentialsValid(false))
      .json<{ name: string }>()
      .then(it => {
        setSessionName(it.name)
        setCredentialsValid(true)
      })
  }, [sessionId, secret])

  useEffect(() => {
    if (!credentialsValid) {
      return
    }
    backend.get(`/session/${sessionId}/participant/me`)
      .json<{ name?: string }>()
      .then(it => setOwnName(it.name))
  }, [sessionId, credentialsValid])

  return <Page title={credentialsValid === undefined ? undefined : sessionName}>
    {credentialsValid && ownName && <SessionView/>}
    {credentialsValid && !ownName && <SetName/>}
    {credentialsValid !== undefined && !credentialsValid && <h2>The session link is not valid (╯°益°)╯彡┻━┻</h2>}
    {credentialsValid === undefined && <Spinner size='page' color='primary'/>}
  </Page>
}
