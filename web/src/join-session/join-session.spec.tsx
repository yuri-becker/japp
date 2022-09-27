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

import { fireEvent, render, RenderResult } from '@testing-library/react'
import JoinSession from './join-session'

describe('JoinSession', () => {
  let sut: RenderResult
  const inputSessionLink = async (value: any) => {
    fireEvent.change(await sut.findByLabelText('Session Link'), { target: { value } })
  }
  const isJoinSessionButtonDisabled = async () => (await sut.findByText('Join session') as HTMLButtonElement).disabled
  beforeEach(() => {
    sut = render(<JoinSession/>)
  })
  it('should disable button when link input is empty', async () => {
    await inputSessionLink('')
    expect(await isJoinSessionButtonDisabled()).toBeTruthy()
  })
  it('should enable button when the link input is a valid session link', async () => {
    await inputSessionLink(
      'http://localhost/app/session/874b353b-7799-4809-9780-50bc0f9324c1/2ae216ab91f7926524df8e9802b96d7c')
    expect(await isJoinSessionButtonDisabled()).toBeFalsy()
  })
  it('should disable button when the link input is missing the origin', async () => {
    await inputSessionLink('/app/session/874b353b-7799-4809-9780-50bc0f9324c1/2ae216ab91f7926524df8e9802b96d7c')
    expect(await isJoinSessionButtonDisabled()).toBeTruthy()
  });

  it('should disable button when the link input is missing the password', async () => {
    await inputSessionLink('http://localhost/app/session/874b353b-7799-4809-9780-50bc0f9324c1/')
    expect(await isJoinSessionButtonDisabled()).toBeTruthy()
  });
  it('should disable button when the link input pointing to a different endpoint', async () => {
    await inputSessionLink(
      'http://localhost/app/session-thingy/874b353b-7799-4809-9780-50bc0f9324c1/2ae216ab91f7926524df8e9802b96d7c')
    expect(await isJoinSessionButtonDisabled()).toBeTruthy()
  });
})
