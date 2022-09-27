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
import React, { lazy, Suspense } from 'react'
import { Route, Routes as R } from 'react-router-dom'
import { Page } from './common/page'
import { Spinner } from './common/spinner'

const CreateSession = lazy(() => import('./create-session/create-session'))
const JoinSession = lazy(() => import('./join-session/join-session'))
const Session = lazy(() => import('./session/session'))

const NotFound = () => <Page>
  <h1>404</h1>
  <p>The page you were looking for could not be found.</p>
  <a href="/">
    <button>Go Home</button>
  </a>
</Page>

const SuspenseFallback = () => <Page><Spinner size="page" color="primary"></Spinner></Page>

const Routes = () => <Suspense fallback={<SuspenseFallback/>}>
  <R>
    <Route path="/join-session" element={<JoinSession/>}/>
    <Route path="/create-session" element={<CreateSession/>}/>
    <Route path="/session/:sessionId/:secret" element={<Session/>}></Route>
    <Route path="*" element={<NotFound/>}/>
  </R>
</Suspense>

export default Routes
