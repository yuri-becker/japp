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

import { FC, PropsWithChildren } from 'react'

export const Page: FC<PropsWithChildren<{ title?: string, subtitle?: string }>> =
  ({ title, subtitle, children }) =>
    <>
      <header>
        <h1>{title ?? 'JAPP'}</h1>
        <p>{subtitle ?? ''}</p>
      </header>
      <main>{children}</main>
    </>
