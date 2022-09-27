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

import { useEffect, useState } from 'react'
import { Input } from '../common/input'
import { isSessionLinkValid } from './is-session-link-valid'
import { Page } from '../common/page'

export default () => {
  const [sessionLink, setSessionLink] = useState<string | undefined>('')
  const [sessionLinkValid, setSessionLinkValid] = useState<boolean | undefined>(undefined)
  useEffect(() => setSessionLinkValid(sessionLink ? isSessionLinkValid(sessionLink.trim()) : undefined), [sessionLink])

  return <Page subtitle="Join a sesssion">
    <Input
      name="session-link"
      label={<>Session Link</>}
      hint={<>Please insert the Session Link someone from your team has sent you</>}
      icon={<i className="icons8">&#xf114;</i>}
      errorMessage={sessionLinkValid === false ? 'This is not a valid session link :(' : undefined}
      value={sessionLink}
      change={setSessionLink}
    />
    <a href={sessionLink}><button disabled={!sessionLinkValid}>
      <i className="icons8">&#xf10e;</i>Join session</button>
    </a>
  </Page>
}
