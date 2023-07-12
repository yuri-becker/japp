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
import { useNavigate } from 'react-router-dom'
import { Input } from '../common/input'
import { Page } from '../common/page'
import { Spinner } from '../common/spinner'
import { useCreateSession } from './api/use-create-session'

const CreateSession = () => {
  const navigate = useNavigate()
  const [sessionName, setSessionName] = useState<string | undefined>('')
  const [createSession, loading] = useCreateSession()

  const onCreateClick = () => createSession({ name: sessionName ?? undefined })
    .then(createdSession => navigate(`/session/${createdSession.id}/${createdSession.secret}`))

  return <Page subtitle="Create a session">
    <Input
      name="session-name"
      icon={<i className="icons8">&#xf10b;</i>}
      label={<>Session Name</>}
      hint={<>...or use a random session name by leaving this empty.</>}
      value={sessionName}
      change={setSessionName}
    />
    <button onClick={onCreateClick} disabled={loading}>
      {loading && <Spinner size={'inline'}/>}
      <i className="icons8">&#xf106;</i>
      Create session
    </button>
  </Page>
}

export default CreateSession
