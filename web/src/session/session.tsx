import React, { useEffect, useState } from 'react'
import { useBackend } from '../common/use-backend'
import { useSessionStore } from './session-state'
import { SetName } from './set-name'
import { useSessionParams } from './use-session-params'
import { Page } from '../common/page'
import { SessionView } from './session-view'
import { Spinner } from '../common/spinner'
import { useEventStream } from './use-event-stream'

const Session = () => {
  const { sessionId, secret } = useSessionParams()
  const backend = useBackend()
  const { sessionName, setSessionName, ownName, setOwnName } = useSessionStore()
  const [credentialsValid, setCredentialsValid] = useState<boolean | undefined>()
  const [openStream, closeStream] = useEventStream()

  useEffect(() => {
    if (!sessionId || !secret) {
      return
    }
    backend.post(undefined, `/session/${sessionId}/participant?secret=${secret}`)
      .notFound(() => setCredentialsValid(false))
      .unauthorized(() => setCredentialsValid(false))
      .json<{ name: string }>()
      .then(loginResponse => {
        return !loginResponse
          ? Promise.resolve()
          : Promise.all([
            backend.get(`/session/${sessionId}/participant/me`).json<{ name?: string }>(),
            openStream()
          ]).then(([nameResponse]) => {
            setOwnName(nameResponse.name)
            setSessionName(loginResponse.name)
            setCredentialsValid(true)
          })
      })
    return () => closeStream()
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [sessionId, secsret])

  useEffect(() => {
    if (!credentialsValid) {
      return
    }
    backend.get(`/session/${sessionId}/participant/me`)
      .json<{ name?: string }>()
      .then(it => setOwnName(it.name))
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [sessionId, credentialsValid])

  return <Page title={credentialsValid === undefined ? undefined : sessionName}>
    {credentialsValid && ownName && <SessionView/>}
    {credentialsValid && !ownName && <SetName/>}
    {credentialsValid !== undefined && !credentialsValid && <h2>The session link is not valid (╯°益°)╯彡┻━┻</h2>}
    {credentialsValid === undefined && <Spinner size='page' color='primary'/>}
  </Page>
}

export default Session
