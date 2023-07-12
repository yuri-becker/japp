import { useEffect, useState } from 'react'
import { Page } from '../common/page'
import { Spinner } from '../common/spinner'
import { useBackend } from '../common/use-backend'
import { ParticipantResponse } from './api/participant-response'
import { useEventStream } from './api/use-event-stream'
import { SessionView } from './session-view'
import { SetName } from './set-name'
import { useOwnParticipant, useSessionName, useSessionStore } from './state/session.state'
import { useSessionParams } from './use-session-params'

const Session = () => {
  const { sessionId, secret } = useSessionParams()
  const backend = useBackend()
  const { setOwnParticipantId } = useSessionStore()
  const sessionName = useSessionName()
  const ownParticipant = useOwnParticipant()
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
            backend.get(`/session/${sessionId}/participant/me`).json<ParticipantResponse>(),
            openStream()
          ]).then(([meResponse]) => {
            setOwnParticipantId(meResponse.id)
            setCredentialsValid(true)
          })
      })
    return () => closeStream()
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [sessionId, secret])

  useEffect(() => {
    if (!credentialsValid) {
      return
    }
    backend.get(`/session/${sessionId}/participant/me`)
      .json<ParticipantResponse>()
      .then(it => setOwnParticipantId(it.id))
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [sessionId, credentialsValid])

  return <Page title={credentialsValid === undefined ? undefined : sessionName}>
    {credentialsValid && ownParticipant?.name && <SessionView/>}
    {credentialsValid && !ownParticipant?.name && <SetName/>}
    {credentialsValid !== undefined && !credentialsValid && <h2>The session link is not valid (╯°益°)╯彡┻━┻</h2>}
    {credentialsValid === undefined && <Spinner size='page' color='primary'/>}
  </Page>
}

export default Session
